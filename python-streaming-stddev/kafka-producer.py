from kafka import KafkaProducer

producer = KafkaProducer(acks='all')

for i in range(100):
    producer.send('hello', value=f'hi {i}'.encode('utf-8'))
producer.flush()
