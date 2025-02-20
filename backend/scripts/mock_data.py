import sqlite3
import uuid
import random
from datetime import datetime, timedelta

def create_mock_slides(db_path: str, count: int):
    conn = sqlite3.connect(db_path)
    cursor = conn.cursor()
    
    filetypes = ["jpg", "png", "gif", "mp4"]
    now = datetime.utcnow()
    
    slides = []
    for i in range(count):
        slide_id = str(uuid.uuid4())
        caption = f"Slide nr. {i}!"
        start_date = now + timedelta(days=random.randint(-10, 10))
        end_date = start_date + timedelta(days=random.randint(1, 10))
        active = random.choice([True, False])
        filetype = random.choice(filetypes)
        
        slides.append((slide_id, caption, start_date, end_date, active, filetype))
    
    cursor.executemany("""
        INSERT INTO slides (id, caption, start_date, end_date, active, filetype)
        VALUES (?, ?, ?, ?, ?, ?)""", slides)
    
    conn.commit()
    conn.close()
    
    print(f"Inserted {count} mock slides.")

if __name__ == "__main__":
    create_mock_slides("../database.db", 10)