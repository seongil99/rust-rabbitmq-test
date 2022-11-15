import pika

# docker run -d --name rabbitmq -p 5672:5672 -p 8080:15672 --restart=unless-stopped rabbitmq:management
# docker run -it --rm --name rabbitmq -p 5672:5672 -p 15672:15672 rabbitmq:3.11-management

HOST_NAME = 'localhost'
QUEUE_NAME = 'test_queue'


def callback(ch, method, properties, body):
    print('Received: ', body)


connection = pika.BlockingConnection(
    pika.ConnectionParameters(host=HOST_NAME))
channel = connection.channel()
channel.queue_declare(queue=QUEUE_NAME)
channel.basic_consume(
    queue=QUEUE_NAME, on_message_callback=callback, auto_ack=True)
channel.start_consuming()
