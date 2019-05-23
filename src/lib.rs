#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Object {
    Note(NoteData),
    LongNote(NoteData),
    Obstacle(ObstacleData),
}

impl Object {
    pub fn index(&self) -> i32 {
        match self {
            Object::Note(note) => note.index,
            Object::LongNote(note) => note.index,
            Object::Obstacle(obs) => obs.index,
        }
    }

    pub fn set_index(&mut self, index: i32) {
        match self {
            Object::Note(note) => note.index = index,
            Object::LongNote(note) => note.index = index,
            Object::Obstacle(obs) => obs.index = index,
        }
    }
    
    pub fn id(&self) -> i32 {
        match self {
            Object::Note(note) => note.id,
            Object::LongNote(note) => note.id,
            Object::Obstacle(obs) => obs.id,
        }
    }

    pub fn set_id(&mut self, id: i32) {
        match self {
            Object::Note(note) => note.id = id,
            Object::LongNote(note) => note.id = id,
            Object::Obstacle(obs) => obs.id = id,
        }
    }

    pub fn time(&self) -> f32 {
        match self {
            Object::Note(note) => note.time,
            Object::LongNote(note) => note.time,
            Object::Obstacle(obs) => obs.time,
        }
    }

    pub fn set_time(&mut self, time: f32) {
        match self {
            Object::Note(note) => note.time = time,
            Object::LongNote(note) => note.time = time,
            Object::Obstacle(obs) => obs.time = time,
        }
    }

    pub fn mirrored(&self, count: i32) -> Self {
        let mut new = self.clone();
        new.set_index(count - 1 - self.index());
        new
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum NoteDirection {
    Up,
    Down,
    Left,
    Right,
    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
    Any,
    None,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum NoteLayer {
    Base,
    Upper,
    Top,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum NoteType {
    NoteA,
    NoteB,
    GhostNote,
    Bomb,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct NoteData {
    pub typ: NoteType,
    pub time: f32,
    pub index: i32,
    pub id: i32,
    pub dir: NoteDirection,
    pub layer: NoteLayer,
    pub start_layer: NoteLayer,
    pub flip_index: i32,
    pub flip_y: f32,
    pub time_to_next: f32,
    pub time_to_prev: f32,
}

impl NoteData {
    pub fn set_flip_to(&mut self, target: &NoteData) {
        self.flip_index = target.flip_index;
        self.flip_y = if self.index > target.index { 1f32 } else { -1f32 };
        if (self.index > target.index && self.layer < target.layer) || 
           (self.index < target.index && self.layer > target.layer) {
            self.flip_y *= -1f32;
        }
    }

    pub fn switch(&mut self) {
        self.typ = match self.typ {
            NoteType::NoteA => NoteType::NoteB,
            NoteType::NoteB => NoteType::NoteA,
            typ => typ,
        };
    }

    pub fn mirror(&mut self) {
        use NoteDirection::*;

        self.dir = match self.dir {
            Left => Right,
            Right => Left,
            UpLeft => UpRight,
            UpRight => UpLeft,
            DownLeft => DownRight,
            DownRight => DownLeft,
            dir => dir,
        };
    }

    pub fn mirror_index(&mut self, count: i32) {
        self.index = count - 1 - self.index;
        self.flip_index = count - 1 - self.flip_index;
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum ObstacleType {
    FullHeight,
    Top,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ObstacleData {
    pub time: f32,
    pub index: i32,
    pub id: i32,
    pub typ: ObstacleType,
    pub dur: f32,
    pub width: i32,
}

impl ObstacleData {
    pub fn mirror(&mut self, count: i32) {
        self.index = count - self.width - self.index;
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
#[repr(i32)]
pub enum EventType {
    Event0,
    Event1,
    Event2,
    Event3,
    Event4,
    Event5,
    Event6,
    Event7,
    Event8,
    Event9,
    Event10,
    Event11,
    Event12,
    Event13,
    Event14,
    Event15,
    VoidEvent = -1,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct EventData {
    pub typ: EventType,
    pub time: f32,
    pub val: i32,
}

pub fn bpm_time_to_real(mut num: f32, bpm: f32, shuffle: f32, shuffle_period: f32) -> f32 {
    if shuffle_period > 0f32 && ((num * (1f32 / shuffle_period)).trunc() as i32) % 2 == 1 {
        num += shuffle * shuffle_period;
    }

    if bpm > 0f32 {
        num = (num / bpm) * 60f32;
    }

    num
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bpm_time() {
        assert_eq!(bpm_time_to_real(10f32, 60f32, 1f32, 2f32), 12f32); // arbitrary values compared with c#
    }
}
