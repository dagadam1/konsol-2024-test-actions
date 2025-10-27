"""
Adds a new user with a random uuid id and admin status 
to the `users` table in the database.
Usage:
    python add_authenticated_user.py <user_email> <db_path>
Arguments:
    <user_email>    The email address of the user to add.
    <db_path>       Path of the database relative to where the script is ran.
"""

import sys
import sqlite3
import uuid

def add_user(db_path: str, user: str):
    """Add a user with email `user` to the databse table `users` with a random uuid id
    and admin status."""
    conn = sqlite3.connect(db_path)
    cursor = conn.cursor()
    
    id = str(uuid.uuid4())
    
    cursor.execute("""
        INSERT INTO users (id, email, admin)
        VALUES (?, ?, ?)""", (id, user, True))
    
    conn.commit()
    conn.close()
    
if __name__ == "__main__":
    user = sys.argv[1]
    db_path = sys.argv[2]
    add_user(db_path, user)