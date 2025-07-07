import tensorflow as tf
from waymo_open_dataset.protos import scenario_pb2

# Assuming you have a scenario TFRecord file downloaded.
# Replace 'path/to/your/scenario.tfrecord' with the actual path to your file.
# For example, a common naming convention for scenario files is
# 'uncompressed_scenario_training_20s_training_20s.tfrecord-00000-of-01000'
FILENAME = '/home/autod/Downloads/all_waymo_data/uncompressed-scenario-testing-testing.tfrecord-00000-of-00150'

# Create a TFRecordDataset.
# No compression_type needed for uncompressed files.
raw_dataset = tf.data.TFRecordDataset([FILENAME])

# Iterate through the dataset to read a scenario record
for raw_record in raw_dataset.take(1): # .take(1) reads only the first record
    proto_string = raw_record.numpy()
    proto = scenario_pb2.Scenario()
    proto.ParseFromString(proto_string)
    print("Successfully read a scenario record:")
    print(proto)
    break # Exit after processing the first scenario

