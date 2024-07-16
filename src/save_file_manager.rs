//! currently just a mockup of what the save file manager will look like
use std::{
    fs::OpenOptions,
    io::{self, BufRead, Write},
};

use erased_serde::{Serialize, Serializer};
use serde::Deserialize;

trait SaveFile {
    fn get_components(&self) -> Vec<Box<dyn SaveFileComponent>>;
}

trait SaveFileComponent<'a> {
    fn serialize_comp(&self) -> String;
    fn deserialize_comp(data: &'a str) -> Self
    where
        Self: Sized;
}

impl<'a, T> SaveFileComponent<'a> for T
where
    T: Serialize + Deserialize<'a>,
{
    fn serialize_comp(&self) -> String {
        let mut buf = Vec::new();
        {
            let serializer = &mut serde_json::Serializer::new(&mut buf);
            let mut serializer = Box::new(<dyn Serializer>::erase(serializer));

            self.erased_serialize(&mut serializer).unwrap();
        }
        String::from_utf8(buf).expect("Failed to serialize component")
    }

    fn deserialize_comp(data: &'a str) -> T {
        let deserialized: T = serde_json::from_str(data).unwrap();

        deserialized
    }
}

impl dyn SaveFile {
    fn save(&self, file_name: &str) -> io::Result<()> {
        std::fs::File::create(file_name)?;
        let file = OpenOptions::new().append(true).open(file_name)?;
        let mut writer = io::BufWriter::new(file);

        let components = self.get_components();
        for component in components {
            writer.write(b":::")?;
            // serialize component and write to file
            let serialized = component.serialize_comp();
            writer.write(serialized.as_bytes())?;
        }
        Ok(())
    }

    /*
    fn load(&self, file_name: &str) -> io::Result<Vec<Box<dyn SaveFileComponent>>> {
        let file = OpenOptions::new().read(true).open(file_name)?;
        let reader = io::BufReader::new(file);

        let seperated: Vec<String> = reader
            .lines()
            .map(|line| line.unwrap())
            .collect::<String>()
            .split(":::")
            .map(|s| s.to_string())
            .collect();

        let mut components = Vec::new();

        for component in seperated {
            let str = component.as_str();

            let deserialized: Box<dyn SaveFileComponent> = SaveFileComponent::deserialize_comp(str);
            components.push(deserialized);
        }

        Ok(components)
    }
    */
}

mod test {
    use super::*;
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    struct Transform {
        x: f32,
        y: f32,
        z: f32,
        rotation: f32,
        scale: f32,
        origin_x: f32,
        origin_y: f32,
    }

    struct SaveFileMock {
        transform: Transform,
    }

    #[test]
    fn test_transform() {
        let transform = Transform {
            x: 1.0,
            y: 2.0,
            z: 3.0,
            rotation: 4.0,
            scale: 5.0,
            origin_x: 6.0,
            origin_y: 7.0,
        };

        let serialized = transform.serialize_comp();
        let deserialized = Transform::deserialize_comp(&serialized);

        assert_eq!(transform, deserialized);
    }

    #[test]
    fn test_save_file() {
        let transform = Transform {
            x: 1.0,
            y: 2.0,
            z: 3.0,
            rotation: 4.0,
            scale: 5.0,
            origin_x: 6.0,
            origin_y: 7.0,
        };

        let save_file = SaveFileMock { transform };

        save_file.save();
        save_file.load();
    }
}
