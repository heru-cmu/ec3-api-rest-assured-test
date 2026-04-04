# Petstore API Automation (Katalon Studio)

Project ini berisi automation testing untuk **Petstore API** menggunakan **Katalon Studio**. Fokus pada CRUD User endpoints dengan verifikasi schema JSON (toleran & strict).

## 🤔 Reflection Questions
- Awalnya saya kelupaan untuk membuat **Test Cases**, karena disangka cukup dengan membuat **Object Repository** dan mengubah JSON request body.  
  → Pelajaran: Object Repository hanya menyimpan endpoint & request, tapi eksekusi + verifikasi harus ada di Test Case.  

- Saya kemudian memikirkan bagaimana dalam **1 Test Case Negatif** bisa terdapat banyak variasi input invalid agar lebih efisien, sehingga tidak perlu membuat terlalu banyak Test Case di Katalon.  
  → Pelajaran: Looping skenario negatif dalam satu Test Case lebih hemat, dan bisa ditutup dengan **summary hasil** (BUG vs OK).  

- Dengan pendekatan ini, struktur project jadi lebih rapi: **Positif** dipisah, **Multi Negatif** dikumpulkan, lalu dijalankan berurutan lewat Test Suite CRUD.  
  → Pelajaran: Pisah Positif & Negatif membuat hasil lebih jelas, dan Test Suite bisa mengatur flow end-to-end.

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
... (bagian verifikasi yang sudah ada tetap)

## 📂 Test Cases
- **Positif**
  - POST User (Positif)
  - PUT User (Positif)
  - GET User (Positif)
  - DELETE User (Positif)
- **Multi Negatif**
  - POST User (Multi Negatif)
  - PUT User (Multi Negatif)
  - GET User (Multi Negatif)
  - DELETE User (Multi Negatif)

Setiap test case negatif menggunakan looping untuk menjalankan beberapa variasi input invalid sekaligus, lalu menampilkan **summary hasil** (BUG vs OK).

## 📂 Test Suite
- **Run Test Suite CRUD Flow API - User (POST-PUT-GET-DELETE)**
  - Menjalankan semua test case CRUD secara berurutan:
    1. POST User (Positif)
    2. POST User (Multi Negatif)
    3. PUT User (Positif)
    4. PUT User (Multi Negatif)
    5. GET User (Positif)
    6. GET User (Multi Negatif)
    7. DELETE User (Positif)
    8. DELETE User (Multi Negatif)
  - Positif menggunakan `STOP_ON_FAILURE` (berhenti jika gagal).
  - Multi Negatif menggunakan `CONTINUE_ON_FAILURE` (tetap lanjut walau ada error).

## 📂 Reporting
- Hasil eksekusi Test Suite otomatis tersimpan di folder `Reports/`.
- Log menampilkan detail request (HAR + cURL) dan hasil assertion.
- Summary di akhir setiap test case negatif memudahkan identifikasi skenario yang diterima/ditolak API.
