import tkinter as tk
from datetime import datetime
from tkinter import ttk
import pika

HOST_NAME = 'localhost'
QUEUE_NAME = 'test_queue'


def send_message():
    connection = pika.BlockingConnection(
        pika.ConnectionParameters(host=HOST_NAME))
    channel = connection.channel()
    channel.queue_declare(queue=QUEUE_NAME)
    channel.basic_publish(
        exchange='', routing_key=QUEUE_NAME, body='Hello World!')
    connection.close()


class App(tk.Tk):
    def __init__(self):
        super().__init__()
        self.title('Test')
        self.geometry('400x400')
        self.resizable(False, False)
        self.protocol('WM_DELETE_WINDOW', self.on_closing)

        self.connection = pika.BlockingConnection(
            pika.ConnectionParameters(host=HOST_NAME))
        self.channel = self.connection.channel()
        self.channel.queue_declare(queue=QUEUE_NAME)

        self.text = tk.Text(self, width=30, height=10)
        self.text.pack()

        self.button = ttk.Button(self, text='Send', command=self.send)
        self.button.pack()

    def send(self):
        self.channel.basic_publish(exchange='',
                                   routing_key=QUEUE_NAME,
                                   body=datetime.now().strftime('%Y-%m-%d %H:%M:%S'))
        self.text.insert(tk.END, 'Sent: ' +
                         datetime.now().strftime('%Y-%m-%d %H:%M:%S'))

    def on_closing(self):
        self.connection.close()
        self.destroy()


if __name__ == '__main__':
    app = App()
    app.mainloop()
