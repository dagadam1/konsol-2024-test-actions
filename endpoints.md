## Endpoints

---
### `POST /api/screen/slides/save`

**Description:**  
Uploads a new slide to the database and saves its image.

**Request:**  
- **Content-Type:** `multipart/form-data`
- **Form Fields:**
  - `caption` (string)
  - `start` (string): Start date, format: `YYYY-MM-DD`.
  - `end` (string): Enda date, format: `YYYY-MM-DD`.
  - `visible` (boolean): Visibility of the slide.
  - `imageFile` (file): The image file for the slide.

**Response:**  
- **Status Code:** `201 Created`
- **Body:** JSON object of slide created, same format as below.

**Errors:**  
- `500 Internal Server Error`: If saving the image or inserting the slide into the database fails.

---

### `GET /api/screen/slides`

**Description:**  
Retrieves all slides stored in the database, without their images.

**Request:**  
- No parameters.

**Response:**  
- **Status Code:** `200 OK`
- **Body:** JSON array of slide objects:
    ```json
  {
    "id": "string",          // UUID of the slide
    "caption": "string",     // Caption of the slide
    "start_date": "string",  // Start date in ISO 8601 format (e.g., "YYYY-MM-DDTHH:MM:SS")
    "end_date": "string",    // End date in ISO 8601 format (e.g., "YYYY-MM-DDTHH:MM:SS")
    "active": true,          
    "filetype": "string"     // e.g., "jpeg", "png"
  }
    ```

**Errors:**  
- `500 Internal Server Error`: If retrieving slides from the database fails.

---

### `GET /api/screen/slides/images/<id>`

**Description:**  
Serves static image files for slides from the directory specified by `SLIDE_IMAGE_DIR`.

**Request:**  
- **Path Parameter:**
  - `id` (string): The id of the slide.

**Response:**  
- **Status Code:** `200 OK`
- **Body:** The requested image file.

**Errors:**  
- `404 Not Found`: If the requested image file does not exist.

---