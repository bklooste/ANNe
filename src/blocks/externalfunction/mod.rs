//! Provides function block



// let dl = DynamicLibrary::open(Some(dl_path));
//  match dl {
//    Ok(dll) => {
//      // Try loading symbols. Note that because of the api we can't
//      // directly pass T as fn(c_int) -> c_int, because the return is
//      // a pointer, not a pointer to a pointer.
//      unsafe{
//        rtn.foo = match dll.symbol::<c_void>("foo") {
//          Ok(symbol) => Some(transmute(symbol)),
//          Err(_) => None
//        };
//        rtn.bar = match dll.symbol::<c_void>("bar") {
//          Ok(symbol) => Some(transmute(symbol)),
//          Err(_) => None
//        };
//        trace!("Read: {}", dll.symbol::<c_void>("foo"));
//        trace!("Read: {}", dll.symbol::<c_void>("bar"));
//        rtn.lib = Some(dll);
//      }
//    }
