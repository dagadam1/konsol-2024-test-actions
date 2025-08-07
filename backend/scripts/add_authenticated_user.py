"""
Adds a new user with a random uuid id and admin status 
to the `users` table in the database at '../database.db'.
Usage:
    python add_authenticated_user.py <user_email>
Arguments:
    <user_email>    The email address of the user to add.
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
    add_user("../database.db", user)