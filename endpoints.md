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
### `POST /api/auth/verify`

**Description:**  
Authenticates a user with a `client_id` from Google OAuth. Only users with emails in the database table `Users` are allowed to authenticate.

**Request:**  
- **Body:** JSON object (AuthRequest struct in Actix):
```json
{
  "client_id": "string",
}
```

**Response:**  
- **Status Code:** `200 OK`
- **Body:** JSON object (AuthenticatedUser struct in Actix):
```json
{
  "email": "string",
  "permission": "User" | "Admin"
}
```

**Errors:**  
- `401 Unauthorized`: If unable to authenticate.

---
### `GET /api/auth/status`

**Description:**  
Checks the authentication status of the current session.

**Response:**  
- **Status Code:** `200 OK`
- **Body:** JSON object (AuthenticatedUser struct in Actix):
```json
{
  "email": "string",
  "permission": "User" | "Admin"
}
```
**Cookies:**
- Sets a session cookie if authenticated.


**Errors:**  
- `401 Unauthorized`: If session is not authenticated.

---
### `POST /api/auth/logout`

**Description:**  
Clears the authentication status of the current session.

**Response:**  
- **Status Code:** `200 OK`

**Errors:**  
- `401 Unauthorized`: If session is not authenticated.

---
### `POST /api/auth/add_user`

**Description:**  
Adds a user to the database. Needs admin permissions.

**Request:**
- **Body:** See AddUserRequest in Actix. Note that the enum PermissionLevel is serialized into its fields case sensitively. That means permission is case sensitive. Permission is either "Admin" or "User". Setting "user", for example, will generate an error. 

**Response:**  
- **Status Code:** `200 OK`
- **Body:** See User struct in Actix.

**Errors:**  
- `401 Unauthorized`: If session is not authenticated.
- `403 Forbidden`: If session does not have admin permissions.

---
### `POST /api/auth/remove_user`

**Description:**  
Removes a user from the database. Needs admin permissions.

**Request:**
- **Body:** See RemoveUserRequest in Actix.

**Response:**  
- **Status Code:** `200 OK`

**Errors:**  
- `401 Unauthorized`: If session is not authenticated.
- `403 Forbidden`: If session does not have admin permissions.

---
### `GET /api/auth/list_users`

**Description:**  
List all allowed users and their permissions. Needs admin permissions.

**Response:**  
- **Status Code:** `200 OK`
- **Body:** JSON array of User structs (see Actix).

**Errors:**  
- `401 Unauthorized`: If session is not authenticated.
- `403 Forbidden`: If session does not have admin permissions.
