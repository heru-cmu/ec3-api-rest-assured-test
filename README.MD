# Petstore API Automation (Katalon Studio)

Project ini berisi automation testing untuk **Petstore API** menggunakan **Katalon Studio**. Fokus pada CRUD User endpoints dengan verifikasi schema JSON (toleran & strict).

## 📂 Fitur Utama
- **CRUD User Endpoints**
  - `POST /user` → Create User
  - `GET /user/{username}` → Get User by username
  - `PUT /user/{username}` → Update User
  - `DELETE /user/{username}` → Delete User
- **Verification Scripts**
  - Toleran → menerima variasi tipe numeric (`Integer` vs `Long`)
  - Strict → validasi persis sesuai schema
- **Schema JSON** → disimpan di folder `Include/resources/schema/`
## 📂 Verification & Assertion

### ✅ GET User
- Status Code → 200  
- Field Validation (Strict):  
  - `id` → Long  
  - `username` → String  
  - `firstName` → String  
  - `lastName` → String  
  - `email` → String  
  - `password` → String  
  - `phone` → String  
  - `userStatus` → Integer  
- Required Fields → `id`, `username`, `email`

### ✅ POST User
- Status Code → 200  
- Field Validation (Strict):  
  - `code` → Integer  
  - `type` → String  
  - `message` → String  
- Required Fields → `code`, `type`, `message`

### ✅ PUT User
- Status Code → 200  
- Field Validation:  
  - Toleran → `code` bisa Integer/Long  
  - Strict → `code` harus Integer  
  - `type` → String  
  - `message` → String  
- Required Fields → `code`, `type`, `message`

### ✅ DELETE User
- Status Code → 200  
- Field Validation (Strict):  
  - `code` → Integer  
  - `type` → String  
  - `message` → String (username yang dihapus)  
- Required Fields → `code`, `type`, `message`

### 📌 Assertion Patterns
```groovy
// Status Code
WS.verifyResponseStatusCode(response, 200)
assertThat(response.getStatusCode()).isEqualTo(200)

// Field Type
assert jsonResponse.id instanceof Long
assert jsonResponse.userStatus instanceof Integer
assert jsonResponse.username instanceof String

// Required Field
assert jsonResponse['id'] != null
assert jsonResponse['username'] != null
assert jsonResponse['email'] != null
