use tangram::prelude::*;

fn main() {
    // Create an in-memory dataset for binarry classification
    let dataset = tangram::BinaryClassificationDataset::from_path("path_to_your_dataset").unwrap();

    // Create a configuration for the model.
    let config = tangram::BinaryClassifier::train_config();

    // Train the model using the dataset and the config.
    let model = tangram::BinaryClassifier::train(dataset, config).unwrap();

    // Save the trained model to a file.
    model.write_to_path("path_to_save_model").unwrap();
    

}
