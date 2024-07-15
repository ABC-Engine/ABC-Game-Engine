//! currently just a mockup of what the save file manager will look like
use serde::{Serialize, Serializer};

trait SaveFile {
    fn get_components(&self) -> Vec<Box<dyn SaveFileComponent>>;
}

trait SaveFileComponent {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer;
}

impl<T: Serialize> SaveFileComponent for T {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.serialize(serializer)
    }
}

impl SaveFile {
    fn save(&self) {
        let components = self.get_components();
        for component in components {
            // serialize component and write to file
        }
    }

    fn load(&self) {
        // read file and deserialize components
    }
}
