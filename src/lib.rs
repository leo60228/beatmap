use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use std::io::{self, prelude::*};
use serde::{Serialize, Deserialize};

mod helper;
pub use helper::*;

#[derive(PartialEq, Eq, Clone, Copy, Debug, Hash, Serialize, Deserialize)]
pub struct Pointer {
    #[serde(rename = "FileID")]
    pub file: i32,
    #[serde(rename = "PathID")]
    pub path: i64,
}

impl Pointer {
    pub fn write(&self, writer: &mut impl Write) -> io::Result<()> {
        writer.write_i32::<LittleEndian>(self.file)?;
        writer.write_i64::<LittleEndian>(self.path)?;
        Ok(())
    }

    pub fn read(reader: &mut impl Read) -> io::Result<Self> {
        let file = reader.read_i32::<LittleEndian>()?;
        let path = reader.read_i64::<LittleEndian>()?;

        Ok(Self { file, path, })
    }
}

#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
pub struct Difficulty {
    #[serde(rename = "_difficulty")]
    pub difficulty: i32,
    #[serde(rename = "_difficultyRank")]
    pub rank: i32,
    #[serde(rename = "_noteJumpMovementSpeed")]
    pub note_jump: f32,
    #[serde(rename = "_noteJumpStartBeatOffset")]
    pub note_jump_offset: i32,
    #[serde(rename = "_beatmapData")]
    pub beatmap: Pointer,
}

#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
pub struct Difficulties {
    #[serde(rename = "Array")]
    pub vec: Vec<Difficulty>,
    pub size: u32,
}

impl Difficulties {
    pub fn write(&self, writer: &mut impl Write) -> io::Result<()> {
        writer.write_u32::<LittleEndian>(self.size)?;

        for difficulty in &self.vec {
            difficulty.write(writer)?;
        }

        Ok(())
    }

    pub fn read(reader: &mut impl Read) -> io::Result<Self> {
        let mut vec = Vec::new();
        let size = reader.read_u32::<LittleEndian>()?;

        for _ in 0..size {
            vec.push(Difficulty::read(reader)?);
        }

        Ok(Self { vec, size, })
    }
}

impl Difficulty {
    pub fn write(&self, writer: &mut impl Write) -> io::Result<()> {
        writer.write_i32::<LittleEndian>(self.difficulty)?;
        writer.write_i32::<LittleEndian>(self.rank)?;
        writer.write_f32::<LittleEndian>(self.note_jump)?;
        writer.write_i32::<LittleEndian>(self.note_jump_offset)?;
        self.beatmap.write(writer)?;

        Ok(())
    }

    pub fn read(reader: &mut impl Read) -> io::Result<Self> {
        let difficulty = reader.read_i32::<LittleEndian>()?;
        let rank = reader.read_i32::<LittleEndian>()?;
        let note_jump = reader.read_f32::<LittleEndian>()?;
        let note_jump_offset = reader.read_i32::<LittleEndian>()?;
        let beatmap = Pointer::read(reader)?;

        Ok(Self { difficulty, rank, note_jump, note_jump_offset, beatmap, })
    }
}

#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
pub struct Beatmap {
    #[serde(rename = "GameObject")]
    pub game_obj: Pointer,
    #[serde(rename = "Enabled")]
    pub enabled: u32,
    #[serde(rename = "MonoScript")]
    pub script: Pointer,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "_levelID")]
    pub id: String,
    #[serde(rename = "_songName")]
    pub song_name: String,
    #[serde(rename = "_songSubName")]
    pub song_sub_name: String,
    #[serde(rename = "_songAuthorName")]
    pub song_author_name: String,
    #[serde(rename = "_levelAuthorName")]
    pub author: String,
    #[serde(rename = "_audioClip")]
    pub audio_clip: Pointer,
    #[serde(rename = "_beatsPerMinute")]
    pub bpm: f32,
    #[serde(rename = "_songTimeOffset")]
    pub time_offset: f32,
    #[serde(rename = "_shuffle")]
    pub shuffle: f32,
    #[serde(rename = "_shufflePeriod")]
    pub shuffle_period: f32,
    #[serde(rename = "_previewStartTime")]
    pub preview_start: f32,
    #[serde(rename = "_previewDuration")]
    pub preview_len: f32,
    #[serde(rename = "_coverImageTexture2D")]
    pub cover: Pointer,
    #[serde(rename = "_environmentSceneInfo")]
    pub environment: Pointer,
    #[serde(rename = "_difficultyBeatmapSets")]
    pub difficulties: Difficulties,
}

impl Beatmap {
    pub fn write(&self, writer: &mut impl Write) -> io::Result<()> {
        self.game_obj.write(writer)?;
        writer.write_u32::<LittleEndian>(self.enabled)?;
        self.script.write(writer)?;
        write_aligned_str(writer, &self.name)?;
        write_aligned_str(writer, &self.id)?;
        write_aligned_str(writer, &self.song_name)?;
        write_aligned_str(writer, &self.song_sub_name)?;
        write_aligned_str(writer, &self.song_author_name)?;
        write_aligned_str(writer, &self.author)?;
        self.audio_clip.write(writer)?;
        writer.write_f32::<LittleEndian>(self.bpm)?;
        writer.write_f32::<LittleEndian>(self.time_offset)?;
        writer.write_f32::<LittleEndian>(self.shuffle)?;
        writer.write_f32::<LittleEndian>(self.shuffle_period)?;
        writer.write_f32::<LittleEndian>(self.preview_start)?;
        writer.write_f32::<LittleEndian>(self.preview_len)?;
        self.cover.write(writer)?;
        self.environment.write(writer)?;
        self.difficulties.write(writer)?;
        Ok(())
    }

    pub fn read(reader: &mut impl Read) -> io::Result<Self> {
        let game_obj = Pointer::read(reader)?;
        let enabled = reader.read_u32::<LittleEndian>()?;
        let script = Pointer::read(reader)?;
        let name = read_aligned_str(reader)?;
        let id = read_aligned_str(reader)?;
        let song_name = read_aligned_str(reader)?;
        let song_sub_name = read_aligned_str(reader)?;
        let song_author_name = read_aligned_str(reader)?;
        let author = read_aligned_str(reader)?;
        let audio_clip = Pointer::read(reader)?;
        let bpm = reader.read_f32::<LittleEndian>()?;
        let time_offset = reader.read_f32::<LittleEndian>()?;
        let shuffle = reader.read_f32::<LittleEndian>()?;
        let shuffle_period = reader.read_f32::<LittleEndian>()?;
        let preview_start = reader.read_f32::<LittleEndian>()?;
        let preview_len = reader.read_f32::<LittleEndian>()?;
        let cover = Pointer::read(reader)?;
        let environment = Pointer::read(reader)?;
        let difficulties = Difficulties::read(reader)?;
        Ok(Self { game_obj, enabled, script, name, id, song_name, song_sub_name, song_author_name, author, audio_clip, bpm, time_offset, shuffle, shuffle_period, preview_start, preview_len, cover, environment, difficulties, })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
