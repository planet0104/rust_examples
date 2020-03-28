//监听按键
use std::collections::HashMap;
use minifb::{Key, Window};

pub struct EventManager{
    events: HashMap<Key, bool>,
    key_map: HashMap<Key, bool>,
}

impl EventManager{
    pub fn new() -> EventManager{
        EventManager{
            events: HashMap::new(),
            key_map: HashMap::new()
        }
    }

    pub fn add_key(&mut self, key:Key){
        self.key_map.insert(key, false);
    }

    pub fn get_event(&mut self, key:&Key) -> Option<bool>{
        if !self.events.contains_key(key){
            return None;
        }
        let key_val = *self.events.get(key).unwrap();
        let _ = self.events.remove(&key);
        Some(key_val)
    }

    pub fn update(&mut self, window:&Window){
        for (key, status) in self.key_map.iter_mut(){
            let current_status = window.is_key_down(*key);
            if current_status != *status{
                self.events.insert(*key, current_status);
            }
            *status = current_status;
        }
    }
}