from kafka import KafkaConsumer

consumer = KafkaConsumer('hello', auto_offset_reset='latest')

sum = 0
for msg in consumer:
    print(msg.value.decode('utf8'))
