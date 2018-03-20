use caps::*;
use oci::{LinuxCapabilities, LinuxCapabilityType};

fn to_cap(cap: LinuxCapabilityType) -> Capability {
    unsafe { ::std::mem::transmute(cap) }
}

fn to_linux_cap(cap: Capability) -> LinuxCapabilityType {
    unsafe { ::std::mem::transmute(cap) }
}

fn to_set(caps: &[LinuxCapabilityType]) -> CapsHashSet {
    let mut capabilities = CapsHashSet::new();
    for c in caps {
        capabilities.insert(to_cap(*c));
    }
    capabilities
}

pub fn reset_effective() -> ::Result<()> {
    let mut all = CapsHashSet::new();
    for c in Capability::iter_variants() {
        all.insert(c);
    }
    set(None, CapSet::Effective, all)?;
    Ok(())
}

pub fn drop_privileges(cs: &LinuxCapabilities) -> ::Result<()> {
    let mut all = CapsHashSet::new();
    for c in Capability::iter_variants() {
        all.insert(c);
    }
    debug!("dropping bounding capabilities to {:?}", cs.bounding);
    // drop excluded caps from the bounding set
     for c in all.difference(&to_set(&cs.bounding)) {
       if !cs.bounding.contains(&to_linux_cap(*c)) {
           drop(None, CapSet::Bounding, *c)?;
       }
    }
    // set other sets for current process
    set(None, CapSet::Effective, to_set(&cs.effective))?;
    set(None, CapSet::Permitted, to_set(&cs.permitted))?;
    set(None, CapSet::Inheritable, to_set(&cs.inheritable))?;
    if let Err(e) = set(None, CapSet::Ambient, to_set(&cs.ambient)) {
        warn!("failed to set ambient capabilities: {}", e);
    }
    Ok(())
}
