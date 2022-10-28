pub trait Summary {}

pub fn notify(item1: &impl Summary, item2: &impl Summary) {} // item1 and item2 can have different
                                                             // types, as long as both of them
                                                             // implements the Summary trait

pub fn notify<T: Summary>(item1: &T, item2: &T) {} // item1 and item2 are forced
                                                   // to have the same type
