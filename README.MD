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
