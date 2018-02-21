use dom::bindings::cell::DomRefCell;
use dom::bindings::codegen::Bindings::DogeBinding::{DogeMethods, DogeInit, Wrap as DogeWrap};
use dom::bindings::error::{Error, Fallible};
use dom::bindings::reflector::{Reflector, reflect_dom_object};
use dom::bindings::root::DomRoot;
use dom::bindings::str::DOMString;
use dom::globalscope::GlobalScope;
use dom_struct::dom_struct;
use servo_rand;
use servo_rand::Rng;



#[dom_struct]
pub struct Doge {
    reflector_: Reflector,
    such_list: DomRefCell<Vec<DOMString>>,
}

impl Doge {
    pub fn new_inherited() -> Doge {
        Doge {
            reflector_: Reflector::new(),
            such_list: DomRefCell::new(vec![]),
        }
    }

    pub fn new(global: &GlobalScope) -> DomRoot<Doge> {
        reflect_dom_object(Box::new(Doge::new_inherited()), global, DogeWrap)
    }

    // https://jeenalee.github.io/doge-standard/#dom-doge
    pub fn Constructor(global: &GlobalScope, init: Option<DogeInit>) -> Fallible<DomRoot<Doge>> {
        // Step 1
        let doge = Doge::new(global);
        // Step 2
        if let Some(i) = init {
            for word in i {
                doge.Append(word);
            }
        }
        // Step 3
        Ok(doge)
    }
}

impl DogeMethods for Doge {
    // https://jeenalee.github.io/doge-standard/#dom-doge-append
    fn Append(&self, word: DOMString) -> () {
        *&self.such_list.borrow_mut().push(word);
    }

    // https://jeenalee.github.io/doge-standard/#dom-doge-random
    fn Random(&self) -> Fallible<DOMString> {
        // Step 1
        let list = self.such_list.borrow();
        // Step 2
        if list.len() == 0 {
            return Err(Error::Type("Such list is empty".to_string()));
        } else {
            // Step 3
            let random_index = servo_rand::thread_rng().gen_range(0, list.len());
            return Ok(list[random_index].clone());
        }
    }
    fn Remove(&self, word: DOMString) -> Fallible<()>{
        let mut isFound = false;
        let mut foundAt = 0;
        let mut thisList = self.such_list.borrow_mut();
        for index in 0..thisList.len(){
            if word == thisList[index]{
                isFound = true;
                foundAt = index;
                break;
            }
        }
        if isFound{
            thisList.remove(foundAt);
            return Ok(())
        } else{
            return Err(Error::Type("This word is not present in the list".to_string()));
        }
    }

    fn PrintAll(&self) {
        let list = self.such_list.borrow();
        for x in 0..list.len() {
            println!("{}", list[x]);
        }
    }
}