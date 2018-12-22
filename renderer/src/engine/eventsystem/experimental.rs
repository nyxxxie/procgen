struct Entity {
    id: i32,
}

impl Entity {
    fn new(id: i32) -> Entity {
        return Entity {
            id
        };
    }
}

trait Event1 {
    fn process(&self);
}

impl Event1 for Entity {
    fn process(&self) {
        println!("Event1 triggered for ent {}", self.id);
    }
}

trait Event2 {
    fn process(&self);
}

impl Event2 for Entity {
    fn process(&self) {
        println!("Event2 triggered for ent {}", self.id);
    }
}

//struct EventQueue<T> {
//    //listeners: Vec<T>
//    listener: *T,
//}
//
//impl<T> EventQueue<T> {
//    fn add_listener(&mut self, *T listener) {
//        //self.listeners.push(listener);
//        self.listener = listener;
//    }
//
//    fn dispatch(&self) {
//        println!("Dispatching event:");
//        self.listener.process_event_1();
//    }
//};

fn dispatch_event1(listener: &Event1) {
    listener.process();
}

fn dispatch_event2(listener: &Event2) {
    listener.process();
}

fn main() {
    println!("Holla");

    let ent = Entity::new(1);
    dispatch_event1(&ent as &Event1);
    dispatch_event2(&ent as &Event2);

    //let event1_queue = EventQueue<Event1> { listener: &ent as &Entity};
    //event1_queue.dispatch();

    //test_cast(&ent as &Entity);
}
