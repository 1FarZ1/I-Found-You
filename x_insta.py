# instagram_client.py
import ensta
import sys

def fetch_followers(username, password):
    session = ensta.InstagramSession(username, password)
    followers = session.followers()
    for user in followers:
        print(user.username)

def fetch_following(username, password):
    session = ensta.InstagramSession(username, password)
    following = session.following()
    for user in following:
        print(user.username)

if __name__ == "__main__":
    command = sys.argv[1]
    username = sys.argv[2]
    password = sys.argv[3]

    if command == "followers":
        fetch_followers(username, password)
    elif command == "following":
        fetch_following(username, password)
