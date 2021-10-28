from typing import Optional
from fastapi import FastAPI

app = FastAPI()
messages = []
users = []


@app.get("/")
def read_root():
    return {"type": "ACK", "msg": "hello world!"}


@app.get("/register")
def read_item(name: str):
    users.append({"name": name, "last": 0})
    return {"type": "DONE", "id": len(users) - 1}


@app.get("/updates/{user_id}")
def whats_new(user_id: int):
    last = users[user_id]["last"]
    users[user_id]["last"] = len(messages)
    return {"type": "UPDATES", "list": messages[last:]}


@app.get("/add/{uid}/{text}")
def new_msg(uid: int, text: str):
    messages.append({"by": users[uid]["name"], "text": text})
    return {"type": "ADDED", "id": len(messages) - 1}
