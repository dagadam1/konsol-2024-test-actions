import sqlite3
import uuid
import random
from datetime import datetime, timedelta
import requests
from dotenv import load_dotenv
import sys
import os

load_dotenv()

def create_mock_slides(db_path: str, count: int):
    conn = sqlite3.connect(db_path)
    cursor = conn.cursor()
    
    now = datetime.utcnow()
    
    slides = []
    for i in range(count):
        slide_id = str(uuid.uuid4())
        
        try:
            image_path = f"../{os.environ['IMAGE_PATH']}/{slide_id}.jpg"
        except KeyError:
            sys.exit("Error: IMAGE_DIR environment variable is not set.")
        
        # Get random image from picsum.photos    
        image = requests.get(f"https://picsum.photos/1920/1080").content
            
        with open(image_path, "wb") as image_file:
            image_file.write(image)
            
        caption = f"Slide nr. {i}!"
        start_date = now + timedelta(days=random.randint(-10, 10))
        end_date = start_date + timedelta(days=random.randint(1, 10))
        active = random.choice([True, False])
        
        slides.append((slide_id, caption, start_date, end_date, active, "jpg"))
    
    cursor.executemany("""
        INSERT INTO slides (id, caption, start_date, end_date, active, filetype)
        VALUES (?, ?, ?, ?, ?, ?)""", slides)
    
    conn.commit()
    conn.close()
    
    print(f"Inserted {count} mock slides.")

if __name__ == "__main__":
    create_mock_slides("../database.db", 4)