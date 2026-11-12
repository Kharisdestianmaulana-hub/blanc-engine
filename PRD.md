# Product Requirements Document (PRD)

> **Document Status:** Draft  
> **Last Updated:** 12 November 2026  
> **Product Manager:** TBD  
> **Contributors:** Product Owner, Engineering, Design, QA

---

## Executive Summary

**Product Name:** Blanc Engine

**Product Type:** Desktop No-Code Visual Novel Game Engine

**Technology Stack:** Rust + Iced

**Product Vision:**  
Memberikan cara yang sederhana, terstruktur, dan mudah dipahami bagi siapa pun untuk mengubah cerita menjadi sebuah visual novel tanpa harus menulis kode program.

### Problem Statement

Membuat visual novel secara mandiri dapat menjadi proses yang cukup sulit bagi penulis atau kreator yang tidak memiliki kemampuan pemrograman. Mereka perlu mengatur aset visual, karakter, dialog, musik, efek suara, dan urutan adegan, sementara game engine yang tersedia sering kali membutuhkan pemahaman teknis atau proses scripting.

Hal tersebut menciptakan hambatan bagi kreator yang ingin berfokus pada cerita dan pengalaman pemain dibandingkan implementasi teknis.

### Proposed Solution

Blanc Engine adalah aplikasi desktop open-source yang memungkinkan pengguna membuat visual novel melalui antarmuka visual tanpa menulis kode. Pengguna dapat mengelola aset, memasukkan cerita, dan menyusun adegan menggunakan sistem block-based scene editor yang menampilkan kumpulan action block secara vertikal.

Blanc Engine akan menyediakan workflow bertahap yang memandu pengguna mulai dari membuat proyek hingga menyusun scene dan menghasilkan visual novel yang dapat dimainkan.

### Success Metrics

Target berikut merupakan sasaran awal yang akan dievaluasi setelah MVP tersedia.

- Pengguna dapat membuat proyek visual novel sederhana tanpa menulis kode.
- Pengguna dapat menyelesaikan satu visual novel sederhana menggunakan fitur MVP.
- Pengguna dapat menyusun dialog, karakter, background, dan audio melalui action block.
- Tidak ditemukan bug kritis yang menghalangi proses pembuatan dan menjalankan proyek.
- Pengguna baru dapat memahami workflow utama tanpa harus membaca dokumentasi teknis secara mendalam.

### Launch Target

TBD — Target rilis ditentukan setelah estimasi pengembangan dan validasi MVP selesai.

---

## Background & Context

### Why Now?

Visual novel merupakan media yang menggabungkan cerita, ilustrasi, musik, dan interaksi pemain. Format ini memungkinkan penulis dan kreator menyampaikan cerita melalui pengalaman interaktif.

Namun, proses pembuatan visual novel masih dapat menjadi tantangan bagi kreator yang tidak terbiasa dengan pemrograman.

Blanc Engine dikembangkan untuk mengurangi hambatan tersebut melalui pendekatan no-code dan workflow visual.

Alasan utama pengembangan:

1. Memberikan alternatif pembuatan visual novel yang lebih mudah diakses oleh kreator non-programmer.
2. Mengurangi kebutuhan untuk menulis dan memahami scripting.
3. Menyatukan pengelolaan aset dan penyusunan scene dalam satu aplikasi.
4. Mengembangkan proyek open-source yang dapat terus ditingkatkan melalui kontribusi komunitas.

### User Research & Insights

- **Research Methods:** Belum dilakukan secara formal. Pengembangan awal didasarkan pada identifikasi kebutuhan produk dan perancangan workflow.
- **Key Findings:**
  - Pembuatan visual novel membutuhkan pengelolaan berbagai jenis aset.
  - Cerita perlu diterjemahkan menjadi urutan scene dan action yang terstruktur.
  - Kreator yang tidak memiliki kemampuan pemrograman membutuhkan antarmuka yang mudah dipahami.
  - Pengguna membutuhkan fleksibilitas untuk mengubah scene yang sudah dibuat tanpa harus mengulang seluruh proses.
  - Workflow bertahap dapat membantu pengguna memahami proses pembuatan visual novel.

- **User Quotes:**

  Belum tersedia. Tidak ada wawancara pengguna formal yang digunakan sebagai sumber kutipan.

### Market Analysis

- **Market Size:** Belum dilakukan riset kuantitatif TAM, SAM, dan SOM.
- **Competitive Landscape:**
  - **Ren'Py:** Engine visual novel yang populer dan berbasis scripting.
  - **Visual Novel Maker:** Menyediakan pendekatan visual untuk pembuatan visual novel.
  - **TyranoBuilder:** Menawarkan pembuatan visual novel melalui pendekatan visual dan tanpa kebutuhan scripting secara langsung.
  - **Other General Game Engines:** Engine seperti Godot dan Unity dapat digunakan untuk visual novel, tetapi memiliki cakupan dan kompleksitas yang lebih luas.

- **Differentiation:**
  1. Berfokus secara khusus pada pembuatan visual novel.
  2. Mengutamakan pengalaman no-code.
  3. Menggunakan block-based scene editor dengan susunan action block vertikal.
  4. Memisahkan pengelolaan aset, cerita, dan penyusunan scene menjadi tahapan yang jelas.
  5. Menggunakan Rust dan Iced sebagai fondasi aplikasi desktop.
  6. Dikembangkan sebagai proyek open-source.

---

## Goals & Success Criteria

### Business Goals

Sebagai proyek open-source, Blanc Engine tidak berorientasi pada pendapatan sebagai tujuan utama.

1. Membangun game engine khusus visual novel yang dapat digunakan oleh kreator non-programmer.
2. Membangun komunitas pengguna dan kontributor open-source.
3. Mengembangkan fondasi produk yang dapat diperluas dengan fitur visual novel yang lebih kompleks.
4. Membangun reputasi Blanc Engine sebagai alternatif pembuatan visual novel yang mudah digunakan.

### User Goals

1. Pengguna dapat membuat proyek visual novel baru dengan mudah.
2. Pengguna dapat mengelola seluruh aset proyek dalam satu tempat.
3. Pengguna dapat memasukkan cerita yang telah ditulis sebelumnya.
4. Pengguna dapat menyusun scene menggunakan action block tanpa menulis kode.
5. Pengguna dapat mengatur karakter, background, dialog, musik, dan efek suara.
6. Pengguna dapat melihat hasil pekerjaan melalui preview atau runtime visual novel.
7. Pengguna dapat kembali ke tahapan sebelumnya untuk melakukan perubahan.

### Success Metrics (Detailed)

| Metric | Baseline | Target | Timeframe | Measurement Method |
|---|---|---|---|---|
| Project Creation | Belum tersedia | 100% proses pembuatan proyek berhasil pada pengujian normal | MVP | Functional testing |
| Asset Management | Belum tersedia | Seluruh kategori aset MVP dapat dikelola | MVP | Feature testing |
| Scene Creation | Belum tersedia | Pengguna dapat membuat scene dengan beberapa action block | MVP | Scenario testing |
| No-Code Workflow | Belum tersedia | Proses pembuatan visual novel sederhana tidak membutuhkan penulisan kode | MVP | User testing |
| Project Persistence | Belum tersedia | Proyek dapat dibuka kembali dengan data tetap utuh | MVP | Save/load testing |
| Runtime Preview | Belum tersedia | Scene yang dibuat dapat dijalankan sesuai urutan block | MVP | Runtime testing |
| Critical Bugs | Belum tersedia | 0 bug kritis pada rilis MVP | Sebelum rilis | QA testing |

Catatan: Target merupakan kriteria awal produk, bukan hasil pengujian atau pencapaian yang sudah terbukti.

### Non-Goals (Out of Scope)

Untuk menjaga fokus pengembangan, Blanc Engine tidak menargetkan hal-hal berikut pada MVP:

- Membuat game 3D.
- Menjadi general-purpose game engine.
- Menggantikan software ilustrasi, animasi, atau pengolah audio.
- Menyediakan sistem pemrograman bebas seperti engine berbasis scripting.
- Menyediakan marketplace aset bawaan.
- Menyediakan layanan cloud untuk menyimpan proyek.
- Menyediakan fitur multiplayer.
- Menyediakan sistem distribusi game atau toko game sendiri.
- Menyediakan editor ilustrasi atau pembuatan karakter yang kompleks.
- Menyediakan sistem kolaborasi real-time.

---

## User Personas & Use Cases

### Primary Persona: Independent Story Creator

- **Role:** Penulis cerita, kreator visual novel independen, atau pemula dalam pengembangan game.
- **Goals:**
  - Mengubah cerita menjadi visual novel.
  - Mengatur karakter dan dialog.
  - Menambahkan background dan audio.
  - Menghasilkan game yang dapat dimainkan.
- **Pain Points:**
  - Tidak memahami pemrograman.
  - Kesulitan mengatur berbagai aset.
  - Tidak ingin menghabiskan waktu mempelajari scripting.
  - Membutuhkan workflow yang jelas.
- **Technical Proficiency:** Beginner hingga Intermediate.
- **Context:** Menggunakan aplikasi desktop untuk membuat proyek visual novel secara mandiri.

### Secondary Persona: Visual Novel Developer

- **Role:** Pengembang game independen atau kreator yang sudah memiliki pengalaman membuat visual novel.
- **Goals:**
  - Mempercepat proses penyusunan scene.
  - Mengelola aset dengan lebih terstruktur.
  - Mengurangi pekerjaan manual.
  - Membuat prototipe cerita interaktif.
- **Pain Points:**
  - Pengaturan scene yang berulang.
  - Pengelolaan aset yang tersebar.
  - Proses scripting untuk kebutuhan visual novel sederhana.
- **Technical Proficiency:** Intermediate hingga Advanced.
- **Context:** Menggunakan Blanc Engine sebagai alternatif workflow visual untuk proyek visual novel.

### Key Use Cases

#### Use Case 1: Membuat Proyek Baru

**Actor:** Independent Story Creator

**Preconditions:**
- Aplikasi Blanc Engine sudah dijalankan.
- Pengguna memiliki lokasi penyimpanan yang dapat diakses.

**Flow:**

1. Pengguna membuka Blanc Engine.
2. Sistem menampilkan halaman awal.
3. Pengguna memilih membuat proyek baru.
4. Pengguna menentukan nama proyek dan lokasi folder.
5. Sistem membuat struktur folder proyek.
6. Sistem membuka workflow pengaturan aset.

**Expected Outcome:**  
Proyek visual novel berhasil dibuat pada folder yang dipilih pengguna.

**Edge Cases:**
- Folder tidak memiliki izin akses.
- Nama proyek tidak valid.
- Folder tujuan sudah memiliki proyek.
- Ruang penyimpanan tidak mencukupi.

#### Use Case 2: Mengelola Aset

**Actor:** Independent Story Creator

**Preconditions:**
- Proyek sudah dibuat.
- Pengguna berada pada tahap pengelolaan aset.

**Flow:**

1. Pengguna membuka kategori aset.
2. Pengguna memilih kategori seperti Backgrounds, Characters, BGM, atau SFX.
3. Pengguna mengimpor file aset.
4. Sistem memvalidasi file.
5. Sistem menyimpan atau mengelola referensi aset dalam proyek.
6. Pengguna dapat melihat aset yang berhasil diimpor.

**Expected Outcome:**  
Aset tersedia dan dapat digunakan dalam pembuatan scene.

**Edge Cases:**
- Format file tidak didukung.
- File rusak atau tidak dapat dibaca.
- File duplikat.
- File sumber dipindahkan atau dihapus.

#### Use Case 3: Memasukkan Cerita

**Actor:** Independent Story Creator

**Preconditions:**
- Proyek sudah dibuat.
- Pengelolaan aset telah diselesaikan sesuai kebutuhan workflow.

**Flow:**

1. Pengguna membuka tahap Story.
2. Pengguna memasukkan atau mengimpor cerita yang telah ditulis.
3. Sistem menampilkan konten cerita.
4. Pengguna memeriksa dan menyesuaikan konten.
5. Pengguna menyelesaikan tahap tersebut.
6. Sistem mengizinkan pengguna melanjutkan ke Scene Editor.

**Expected Outcome:**  
Cerita tersedia sebagai sumber dialog dan narasi untuk penyusunan scene.

**Edge Cases:**
- Format impor tidak didukung.
- File cerita kosong.
- Konten tidak dapat dibaca.
- Struktur cerita perlu disesuaikan secara manual.

#### Use Case 4: Membuat Scene

**Actor:** Independent Story Creator

**Preconditions:**
- Proyek telah dibuat.
- Pengguna sudah menyelesaikan tahapan yang diwajibkan.
- Aset dan cerita tersedia sesuai kebutuhan scene.

**Flow:**

1. Pengguna membuka Scene Editor.
2. Pengguna membuat scene baru.
3. Sistem menampilkan area penyusunan action block.
4. Pengguna menambahkan action block.
5. Pengguna mengatur properti block seperti background, karakter, dialog, BGM, dan SFX.
6. Pengguna mengatur urutan action block secara vertikal.
7. Pengguna menyimpan scene.
8. Pengguna dapat membuat scene tambahan.

**Expected Outcome:**  
Scene berhasil dibuat dan tersimpan sebagai bagian dari proyek visual novel.

**Edge Cases:**
- Action block belum memiliki konfigurasi wajib.
- Aset yang digunakan tidak ditemukan.
- Pengguna menghapus block yang masih diperlukan.
- Scene tidak memiliki action yang dapat dijalankan.

#### Use Case 5: Menjalankan Visual Novel

**Actor:** Independent Story Creator

**Preconditions:**
- Minimal satu scene tersedia.
- Scene memiliki konfigurasi yang valid.

**Flow:**

1. Pengguna memilih fitur Preview atau Run.
2. Sistem memvalidasi konfigurasi proyek.
3. Sistem memuat aset yang diperlukan.
4. Runtime menjalankan scene berdasarkan urutan action block.
5. Pengguna melihat hasil visual novel.
6. Pengguna dapat kembali ke editor untuk memperbaiki hasil.

**Expected Outcome:**  
Pengguna dapat melihat dan menguji hasil visual novel tanpa harus melakukan proses pemrograman.

**Edge Cases:**
- Aset tidak ditemukan.
- Konfigurasi block tidak valid.
- Audio gagal dimuat.
- Scene memiliki referensi ke data yang sudah dihapus.

---

## Product Requirements

### Functional Requirements

#### Must Have (P0) - MVP

#### 1. Project Creation & Management

- **Description:** Pengguna dapat membuat proyek baru dengan menentukan nama dan folder penyimpanan. Setiap proyek dibuat sebagai Empty Project murni dengan file manifest utama `<NamaProject>.blanc` (berbasis JSON) dan struktur direktori terorganisir.
- **User Story:** As a creator, I want to create a project in my preferred folder with a recognizable `.blanc` file so that all project data is organized in one location.
- **Acceptance Criteria:**
  - [ ] Pengguna dapat membuat proyek baru (tipe Empty Project pada MVP).
  - [ ] Pengguna dapat menentukan lokasi penyimpanan proyek.
  - [ ] Sistem membuat struktur direktori proyek secara otomatis beserta file manifest `<NamaProject>.blanc` dan `Settings/system_screens.bconfig`.
  - [ ] Sistem memvalidasi nama dan lokasi proyek.
  - [ ] Pengguna dapat membuka kembali proyek yang sudah dibuat melalui file `.blanc` atau folder proyek.

#### 2. Guided Workflow

- **Description:** Aplikasi menyediakan tahapan kerja yang memandu pengguna dari pengaturan aset hingga penyusunan scene.
- **User Story:** As a beginner, I want a guided workflow so that I understand each stage of creating a visual novel.
- **Acceptance Criteria:**
  - [ ] Workflow menampilkan tahapan yang tersedia.
  - [ ] Pengguna dapat berpindah kembali ke tahapan sebelumnya.
  - [ ] Pengguna tidak dapat melanjutkan sebelum persyaratan tahapan saat ini terpenuhi.
  - [ ] Tahapan yang telah diselesaikan tetap dapat dibuka dan diedit.
  - [ ] Perubahan pada tahap sebelumnya tidak menghapus pekerjaan pengguna secara otomatis.
  - [ ] Status penyelesaian setiap tahap dapat diketahui pengguna.

#### 3. Asset Management & Supported Formats

- **Description:** Pengguna dapat mengimpor dan mengelola aset proyek berdasarkan kategori dan format file resmi.
- **User Story:** As a creator, I want to organize my assets by category and import supported media formats easily so that I can use them when creating scenes.
- **Acceptance Criteria:**
  - [ ] Pengguna dapat mengimpor aset dari komputer.
  - [ ] Format gambar didukung: PNG (`.png` dengan transparansi alpha untuk sprite), JPG/JPEG (`.jpg`, `.jpeg` untuk background), dan WebP (`.webp`).
  - [ ] Format audio didukung: OGG Vorbis (`.ogg` untuk looping BGM mulus), WAV (`.wav` untuk SFX rendah latensi), dan MP3 (`.mp3`).
  - [ ] Format font didukung: TTF (`.ttf`) dan OTF (`.otf`).
  - [ ] Sistem menyediakan kategori Backgrounds, Characters, Character Expressions, BGM, SFX, Fonts, UI Assets, dan Other.
  - [ ] Setiap aset diberikan Asset ID unik yang dipetakan di file manifest `.blanc`.
  - [ ] Pengguna dapat melihat daftar aset dan pratinjau thumbnail/audio.
  - [ ] Pengguna dapat menghapus atau mengganti nama aset tanpa merusak referensi adegan.
  - [ ] Sistem memvalidasi integritas file dan menolak format yang tidak didukung dengan pesan yang jelas.
  - [ ] Aset dapat digunakan oleh scene editor.

#### 4. Story Management & Naskah Impor

- **Description:** Pengguna dapat memasukkan atau mengimpor cerita yang sudah ditulis menggunakan format naskah standar untuk digunakan sebagai sumber dialog dan narasi otomatis.
- **User Story:** As a writer, I want to import my story from writing apps (like Scrivener or text editors) using Plain Text, Markdown, or Fountain format so that I don't have to copy-paste line by line.
- **Acceptance Criteria:**
  - [ ] Pengguna dapat memasukkan cerita langsung melalui editor internal (`.bstory`).
  - [ ] Sistem mendukung impor file naskah Plain Text (`.txt`) dan Markdown (`.md`) dengan format sandiwara (`Nama (Ekspresi): "Dialog"`).
  - [ ] Sistem mendukung impor file naskah skenario terbuka **Fountain (`.fountain`)** yang kompatibel dengan aplikasi penulisan profesional (Scrivener, Highland, Fade In, WriterDuet).
  - [ ] Parser otomatis membedakan heading adegan/scene, nama pembicara, ekspresi dalam kurung, dialog, dan narasi.
  - [ ] Sistem menampilkan hasil pratinjau impor naskah sebelum dikonversi menjadi elemen scene.
  - [ ] Konten cerita dapat digunakan pada action block Dialogue dan Narration.
  - [ ] Sistem menyimpan data cerita internal dalam file `.bstory` (format JSON) di dalam subfolder `Story/Chapters/`.

#### 5. Block-Based Scene Editor (Story Scenes)

- **Description:** Editor utama untuk membuat adegan cerita menggunakan kumpulan action block yang tersusun secara vertikal. Setiap scene disimpan sebagai file `.bscene` di dalam folder chapter masing-masing (`Scenes/Chapter_XX/`).
- **User Story:** As a creator, I want to arrange scene actions using visual blocks organized by chapters so that I can build a visual novel without writing code.
- **Acceptance Criteria:**
  - [ ] Pengguna dapat membuat scene baru di dalam chapter yang dipilih.
  - [ ] File scene otomatis disimpan dengan format `.bscene` menggunakan nama slug yang ramah manusia dan ID unik internal.
  - [ ] Pengguna dapat menambahkan action block.
  - [ ] Action block ditampilkan dalam susunan vertikal (bukan flowchart).
  - [ ] Pengguna dapat mengubah urutan block melalui drag-and-drop.
  - [ ] Pengguna dapat menghapus dan menduplikasi block.
  - [ ] Pengguna dapat mengedit konfigurasi setiap block via Inspector.
  - [ ] Pengguna dapat menyimpan perubahan scene.
  - [ ] Sistem dapat memvalidasi konfigurasi block sebelum dijalankan.

#### 6. Action Block Configuration

- **Description:** Setiap action block memiliki konfigurasi visual spesifik sesuai fungsinya.
- **User Story:** As a creator, I want to configure each action block visually so that I can control the presentation of my story.
- **Acceptance Criteria:**
  - [ ] Background block dapat memilih background dari asset library berdasarkan Asset ID.
  - [ ] Character block dapat memilih karakter, ekspresi sprite (**Full Sprite per ekspresi**), dan posisi layar preset (**`Left`**, **`Center`**, **`Right`** dengan opsi offset).
  - [ ] Dialogue block dapat memilih dialog dan karakter pembicara (menampilkan nama dan teks ucapan).
  - [ ] Narration block dapat menampilkan teks narasi cerita tanpa pembicara.
  - [ ] BGM block dapat memilih musik latar dengan pengaturan volume dan looping.
  - [ ] SFX block dapat memilih efek suara satu kali putar.
  - [ ] Transition block dapat mengatur transisi visual (fade in, fade out, dissolve).
  - [ ] Wait block dapat mengatur jeda waktu (detik).
  - [ ] Choice block (dasar) dapat mengatur opsi pilihan dan scene tujuan.
  - [ ] Scene End block dapat menandai akhir scene, berpindah ke scene lain, atau kembali ke Main Menu (*Return to Title*).
  - [ ] Semua konfigurasi dilakukan melalui antarmuka visual tanpa koding.

#### 7. Built-in System Screens (Main Menu, Settings, Save/Load, Pause)

- **Description:** Runtime secara otomatis menyediakan layar sistemik non-gameplay (Main Menu, Settings, Save/Load, In-Game Pause Menu) tanpa mengharuskan pengguna membuat tombol atau logika UI manual. Pengguna cukup mengonfigurasi aset visualnya yang disimpan di `Settings/system_screens.bconfig`.
- **User Story:** As a creator, I want ready-to-use Main Menu and Settings screens so that I don't have to build UI logic from scratch.
- **Acceptance Criteria:**
  - [ ] Runtime otomatis menyediakan Main Menu (Title Screen) dengan tombol New Game, Load Game, Settings, Quit.
  - [ ] Pengguna dapat mengatur gambar background, BGM, logo game, dan Starting Scene untuk Main Menu di Project Settings.
  - [ ] Runtime menyediakan In-Game Pause Menu saat membaca cerita.
  - [ ] Runtime menyediakan layar Settings bawaan (volume BGM/SFX, kecepatan teks, window mode).
  - [ ] Runtime menyediakan sistem Save & Load bawaan dengan slot penyimpanan, thumbnail, dan waktu simpan.
  - [ ] Runtime mendukung **Quick Save (F5)** dan **Quick Load (F8 / F9)**.

#### 8. Runtime & Preview

- **Description:** Sistem menjalankan hasil visual novel (System Screens dan Story Scenes) menggunakan runtime mandiri pada jendela terpisah dengan kontrol navigasi standar visual novel.
- **User Story:** As a creator, I want to preview my project with standard visual novel controls so that I can experience the actual game flow before exporting.
- **Acceptance Criteria:**
  - [ ] Pengguna dapat menjalankan preview dari awal (Main Menu) atau dari scene tertentu.
  - [ ] Mendukung kontrol pemain standar: Spasi / Klik Kiri / Enter (Next), Ctrl (Hold Skip), Tombol H (Hide UI), Esc (Pause), F5 (Quick Save), F8/F9 (Quick Load).
  - [ ] Runtime membaca urutan action block.
  - [ ] Runtime memuat background dan karakter pada posisi yang tepat.
  - [ ] Runtime menampilkan dialog dengan typewriter effect dan styling Markdown sederhana (`**tebal**`, `*miring*`).
  - [ ] Runtime memainkan BGM secara berkesinambungan (*seamless looping* antar-scene).
  - [ ] Runtime menjalankan transisi yang dikonfigurasi.
  - [ ] Sistem memberikan informasi ketika terdapat konfigurasi yang tidak valid.
  - [ ] Pengguna dapat kembali ke editor setelah preview.

#### 9. Project Persistence

- **Description:** Semua data proyek dapat disimpan ke file `<NamaProject>.blanc`, `Settings/system_screens.bconfig`, dan folder terkait serta dibuka kembali.
- **User Story:** As a creator, I want my project to persist between sessions so that I can continue working later.
- **Acceptance Criteria:**
  - [ ] Sistem menyimpan konfigurasi proyek ke file `.blanc`.
  - [ ] Sistem menyimpan struktur scene dan action block dalam file `.bscene`.
  - [ ] Sistem menyimpan referensi aset dengan Asset ID.
  - [ ] Sistem menyimpan data cerita internal ke `.bstory`.
  - [ ] Sistem menyimpan konfigurasi layar sistem ke file `.bconfig`.
  - [ ] Pengguna dapat membuka proyek yang sudah ada.
  - [ ] Data tetap konsisten setelah aplikasi ditutup dan dibuka kembali.
  - [ ] Sistem menangani kesalahan penyimpanan dengan memberikan informasi yang jelas.

#### Should Have (P1) - Post-Launch

#### 1. Starter Project Templates

- **Description:** Menyediakan pilihan template proyek siap pakai (misal: Romance VN Starter, Mystery VN Starter) yang sudah memiliki aset dan alur cerita contoh untuk mempercepat pemula belajar.

#### 2. Non-Gameplay System Screens P1 (Backlog, Splash Screen, Credits Roll)

- **Description:** Menyediakan layar non-gameplay tambahan yang sangat umum di visual novel komersial.
- **User Story:** As a creator, I want dialogue backlog, intro splash screens, and credit rolls to give my visual novel a polished commercial feel.
- **Acceptance Criteria:**
  - [ ] Dialogue Backlog / History: Layar riwayat percakapan untuk membaca ulang teks yang terlewat.
  - [ ] Splash Screen / Intro: Layar penampil logo developer atau disclaimer/peringatan konten sebelum Main Menu.
  - [ ] Credits Roll: Layar bergulir untuk menampilkan nama kontributor dan penutup cerita.

#### 2. Choice & Branching Dialogue (Advanced)

- **Description:** Memungkinkan pengguna membuat percabangan dialog kompleks berbasis variabel dan kondisi.
- **User Story:** As a creator, I want to create complex branching paths influenced by player choices.
- **Acceptance Criteria:**
  - [ ] Pengguna dapat membuat pilihan dialog bercabang banyak.
  - [ ] Pengguna dapat mengatur kondisi keterlihatan pilihan.
  - [ ] Runtime mengevaluasi percabangan sesuai alur cerita.

#### 3. Dialogue Customization

- **Description:** Menyediakan kustomisasi tampilan Dialogue Box (skin, posisi textbox, nameplate layout).

#### 4. Variables & Conditions

- **Description:** Memungkinkan pengguna membuat variabel dan kondisi logika melalui antarmuka visual tanpa menulis kode.

#### 5. Export & Build System

- **Description:** Memungkinkan pengguna menghasilkan build visual novel mandiri (*standalone executable*) untuk platform target.

#### Nice to Have (P2) - Future Consideration

#### 1. Bonus & Extra Screens (CG Gallery, Music Room, Scene Replay)

- **Description:** Layar ekstra yang dapat diakses pemain dari Main Menu:
  - **CG Gallery:** Galeri ilustrasi spesial yang terbuka setelah pemain melewati scene tertentu.
  - **Music Room / BGM Player:** Layar pemutar musik untuk mendengarkan kembali trek BGM yang telah terbuka.
  - **Scene Replay / Memory Room:** Memutar ulang adegan favorit tanpa mengulang cerita dari awal.

#### 2. Localization

- **Description:** Mendukung beberapa bahasa untuk konten visual novel.

#### 3. Plugin System

- **Description:** Menyediakan mekanisme ekstensi komunitas.

#### 4. Advanced Animation

- **Description:** Animasi karakter, efek visual, dan pergerakan sprite lanjutan.

#### 5. Reusable Templates

- **Description:** Template scene atau preset aksi yang dapat digunakan kembali.

---

### Non-Functional Requirements

#### Performance

- Aplikasi harus dapat berjalan secara responsif pada komputer desktop dengan spesifikasi yang wajar.
- Penggunaan memori harus dijaga agar tidak berlebihan.
- Proses membuka proyek tidak boleh memuat seluruh aset berukuran besar ke memori tanpa kebutuhan.
- Proses impor aset harus memberikan feedback kepada pengguna.
- Penyusunan dan pengeditan action block harus tetap responsif ketika jumlah block bertambah.
- Waktu startup dan penggunaan resource akan diukur dan ditetapkan lebih lanjut selama pengembangan.

#### Security & Privacy

- Data proyek disimpan secara lokal pada komputer pengguna.
- Aplikasi tidak mengirimkan data proyek ke server eksternal secara default.
- File yang diimpor harus divalidasi sebelum digunakan.
- Kesalahan membaca file tidak boleh menyebabkan aplikasi kehilangan data proyek.
- Fitur jaringan, telemetri, atau integrasi eksternal harus memiliki kebijakan yang jelas apabila nantinya ditambahkan.
- Tidak ada persyaratan autentikasi akun untuk penggunaan lokal MVP.

#### Scalability

- Struktur proyek harus memungkinkan penambahan fitur tanpa mengubah seluruh arsitektur.
- Sistem harus mendukung proyek dengan banyak scene dan aset.
- Sistem data harus memungkinkan penambahan jenis action block baru.
- Runtime harus dapat dikembangkan untuk mendukung branching, variables, dan kondisi.
- Arsitektur editor dan runtime harus memiliki pemisahan tanggung jawab.

#### Accessibility

- Navigasi utama dapat dilakukan menggunakan keyboard.
- Elemen interaktif harus memiliki label yang jelas.
- Informasi error dan validasi harus mudah dipahami.
- Warna bukan satu-satunya indikator status.
- Kontras teks dan elemen antarmuka harus diperhatikan.
- Dukungan screen reader akan dievaluasi sesuai kemampuan framework dan kebutuhan pengembangan.

#### Browser/Platform Support

- **Application Type:** Desktop Application.
- **Primary Technology:** Rust + Iced.
- **Operating System:** TBD.
- **Initial Platform Target:** TBD.
- **Mobile Application:** Tidak termasuk dalam ruang lingkup produk awal.
- **Web Application:** Tidak termasuk dalam ruang lingkup produk awal.

---

## User Experience & Design

### User Flow Diagram

Alur utama penggunaan Blanc Engine:

```text
Launch Blanc Engine
        |
        v
    Home Page
        |
        v
    Create Project
        |
        v
  Select Project Folder
        |
        v
   Project Creation
        |
        v
   Asset Management
        |
        v
    Story Setup
        |
        v
    Scene Editor
        |
        v
  Create Scene Blocks
        |
        v
   Configure Actions
        |
        v
    Save Project
        |
        v
   Preview / Run
        |
        v
    Review Result
        |
        v
  Edit or Continue
```

Pengguna dapat kembali ke tahapan sebelumnya kapan saja. Namun, pengguna hanya dapat melanjutkan ke tahapan berikutnya setelah menyelesaikan persyaratan pada tahapan saat ini.

### Wireframes/Mockups

Wireframe dan mockup belum ditentukan pada tahap PRD ini.

Layar utama yang perlu dirancang:

1. Home Page
2. Create Project Dialog
3. Project Workspace
4. Asset Management
5. Story Editor
6. Scene Editor
7. Action Block Configuration Panel
8. Preview / Runtime Window
9. Project Settings

#### Scene Editor Concept

Scene Editor menggunakan pendekatan **Block-Based Vertical Editor**.

Struktur visual:

```text
+-----------------------------------------------+
| Scene: Opening                                |
+-----------------------------------------------+
|                                               |
|  +-----------------------------------------+  |
|  | BACKGROUND                              |  |
|  | Select: Room_01                         |  |
|  +-----------------------------------------+  |
|                     |                         |
|  +-----------------------------------------+  |
|  | CHARACTER                               |  |
|  | Character: Alice                        |  |
|  | Expression: Happy                       |  |
|  +-----------------------------------------+  |
|                     |                         |
|  +-----------------------------------------+  |
|  | DIALOGUE                                |  |
|  | Character: Alice                        |  |
|  | Text: "Welcome to my world."             |  |
|  +-----------------------------------------+  |
|                     |                         |
|  +-----------------------------------------+  |
|  | BGM                                     |  |
|  | Track: Morning_Theme                    |  |
|  +-----------------------------------------+  |
|                                               |
|              + Add Action Block               |
|                                               |
+-----------------------------------------------+
```

Setiap block memiliki tampilan, properti, dan fungsi yang sesuai dengan jenis action-nya.

Editor tidak menggunakan flowchart sebagai tampilan utama. Urutan eksekusi scene direpresentasikan oleh susunan block secara vertikal.

### Key Interactions

#### 1. Project Creation

- **Trigger:** Pengguna memilih Create Project.
- **Behavior:** Sistem meminta nama proyek dan lokasi folder.
- **Feedback:** Sistem menampilkan status pembuatan proyek atau informasi error.

#### 2. Asset Import

- **Trigger:** Pengguna memilih Import Asset.
- **Behavior:** Sistem membuka pemilih file dan mengelompokkan aset sesuai kategori.
- **Feedback:** Sistem menampilkan hasil impor, termasuk apabila file tidak valid.

#### 3. Workflow Navigation

- **Trigger:** Pengguna memilih tahapan pada navigasi workflow.
- **Behavior:** Sistem memeriksa status penyelesaian tahap yang diperlukan.
- **Feedback:** Sistem menampilkan tahapan yang dapat dibuka dan alasan apabila pengguna belum dapat melanjutkan.

#### 4. Add Action Block

- **Trigger:** Pengguna memilih Add Action Block.
- **Behavior:** Sistem menampilkan jenis action block yang tersedia.
- **Feedback:** Block baru ditambahkan pada susunan vertikal scene.

#### 5. Configure Action Block

- **Trigger:** Pengguna memilih action block.
- **Behavior:** Sistem menampilkan panel konfigurasi berdasarkan jenis block.
- **Feedback:** Perubahan konfigurasi terlihat pada block dan dapat disimpan.

#### 6. Preview Scene

- **Trigger:** Pengguna memilih Preview atau Run.
- **Behavior:** Sistem memvalidasi dan menjalankan scene melalui runtime.
- **Feedback:** Hasil scene ditampilkan atau sistem menunjukkan konfigurasi yang perlu diperbaiki.

### Design Principles for This Product

1. **No-Code First:** Seluruh proses pembuatan visual novel harus dapat dilakukan melalui UI.
2. **Simplicity:** Antarmuka harus mudah dipahami oleh pengguna pemula.
3. **Progressive Disclosure:** Pengaturan dasar ditampilkan terlebih dahulu, sedangkan konfigurasi lanjutan dapat ditampilkan sesuai kebutuhan.
4. **Visual Clarity:** Setiap action block harus memiliki identitas visual dan fungsi yang jelas.
5. **Workflow Guidance:** Pengguna selalu mengetahui sedang berada di tahapan mana.
6. **Non-Destructive Editing:** Pengguna dapat kembali dan mengubah pekerjaan sebelumnya tanpa kehilangan data secara tidak sengaja.
7. **Consistency:** Komponen antarmuka dan pola interaksi harus konsisten di seluruh aplikasi.
8. **Creator Focus:** Antarmuka harus membantu pengguna berfokus pada cerita dan pengalaman visual novel.

---

## Technical Specifications

### System Architecture

Blanc Engine dirancang sebagai aplikasi desktop dengan teknologi Rust dan Iced.

Arsitektur tingkat tinggi terdiri dari beberapa bagian:

1. **Application Layer**
   - Mengelola lifecycle aplikasi.
   - Mengelola navigasi dan workflow.
   - Menghubungkan seluruh modul.

2. **Project Management**
   - Membuat proyek.
   - Membuka proyek.
   - Menyimpan konfigurasi proyek.
   - Mengelola struktur direktori.

3. **Asset Management**
   - Mengimpor aset.
   - Mengelompokkan aset.
   - Memvalidasi file.
   - Mengelola referensi aset.

4. **Story Management**
   - Mengelola konten cerita.
   - Mengelola dialog dan narasi.
   - Menyediakan data cerita untuk scene editor.

5. **Scene Editor**
   - Membuat scene.
   - Mengelola action block.
   - Mengatur urutan block.
   - Mengelola konfigurasi action.

6. **Runtime**
   - Membaca struktur scene.
   - Mengeksekusi action block.
   - Mengatur alur visual novel.
   - Menampilkan hasil permainan.

7. **Rendering**
   - Menampilkan background.
   - Menampilkan karakter.
   - Menampilkan dialog dan UI visual novel.
   - Menjalankan transisi visual.

8. **Audio**
   - Mengelola BGM.
   - Memainkan SFX.
   - Mengatur audio berdasarkan instruksi scene.

9. **Persistence**
   - Menyimpan dan membaca data proyek.
   - Mengelola konfigurasi dan referensi aset.

10. **Export System**
    - Menyiapkan output visual novel untuk platform target.
    - Termasuk dalam pengembangan lanjutan setelah fondasi runtime tersedia.

Editor dan runtime memiliki tanggung jawab yang berbeda. Editor bertugas membangun dan mengubah data proyek, sedangkan runtime bertugas menjalankan data scene tersebut.

### Data Model

Entitas utama yang diperlukan:

| Entity | Description |
|---|---|
| Project | Informasi utama proyek visual novel (`<NamaProject>.blanc`) |
| ProjectSettings | Konfigurasi umum proyek |
| SystemScreensConfig | Konfigurasi visual layar sistem bawaan (Main Menu, Settings, Save/Load, In-Game Pause) |
| Asset | Referensi file aset |
| AssetCategory | Kategori aset |
| Story | Konten cerita |
| Dialogue | Data dialog |
| Narration | Data narasi |
| Scene | Satu bagian atau rangkaian adegan (Story Scene) |
| ActionBlock | Instruksi yang tersusun vertikal dalam scene cerita |
| Character | Informasi karakter |
| CharacterExpression | Ekspresi karakter |
| RuntimeState | Kondisi permainan saat runtime |

Relasi utama:

- Satu Project memiliki satu file manifest `<NamaProject>.blanc` dan satu SystemScreensConfig.
- Satu Project memiliki banyak Asset.
- Satu Project memiliki banyak Scene.
- Satu Project memiliki satu atau lebih Story data.
- Satu Scene memiliki banyak ActionBlock.
- Satu Character dapat memiliki banyak CharacterExpression.
- ActionBlock dapat menggunakan referensi Asset, Character, atau konten Story.
- Runtime menjalankan SystemScreensConfig saat game dimulai (Main Menu) dan mengeksekusi ActionBlock saat masuk ke Scene.

Struktur data final akan ditentukan pada technical design dan implementasi.

### API Requirements

Blanc Engine merupakan aplikasi desktop lokal. MVP tidak membutuhkan REST API atau layanan backend eksternal.

Interaksi utama dilakukan melalui modul internal aplikasi.

| Interface / Module | Method / Operation | Purpose | Input | Output |
|---|---|---|---|---|
| Project Manager | Create Project | Membuat proyek | Nama dan lokasi proyek | Project data |
| Project Manager | Open Project | Membuka proyek | Project path | Project data |
| Asset Manager | Import Asset | Mengimpor aset | File path dan kategori | Asset reference |
| Story Manager | Import Story | Memasukkan cerita | File atau teks | Story data |
| Scene Manager | Create Scene | Membuat scene | Scene metadata | Scene data |
| Scene Manager | Add Action | Menambahkan action | Scene ID dan action type | ActionBlock |
| Runtime | Run Scene | Menjalankan scene | Scene data | Runtime state |
| Persistence | Save Project | Menyimpan proyek | Project data | Save result |
| Persistence | Load Project | Membaca proyek | Project path | Project data |

### Third-Party Integrations

Belum ada integrasi layanan eksternal yang diwajibkan untuk MVP.

Prioritas awal adalah memaksimalkan kemampuan aplikasi lokal dan meminimalkan ketergantungan pada layanan pihak ketiga.

Pemilihan library tambahan untuk audio, rendering, dan format aset akan dilakukan selama technical design.

### Technical Constraints & Stack Choices

- **Bahasa Pemrograman:** Rust (edisi modern idiomatik).
- **Arsitektur Kode:** **Cargo Workspace** modular terpisah:
  - `crates/blanc_core`: Data models (`.blanc`, `.bscene`, `.bstory`), serialisasi, validasi, dan logic state (murni Rust tanpa dependensi UI).
  - `crates/blanc_editor`: GUI Desktop Editor utama berbasis Iced.
  - `crates/blanc_runtime`: Runtime player mandiri berbasis Iced/Renderer yang dapat di-bundle secara terpisah saat export game.
- **Framework UI:** Iced (GUI deklaratif desktop).
- **Resolusi Kanvas Standar:** Default **`1920 × 1080` (16:9 Full HD)** dengan sistem proportional scaling dan letterbox otomatis.
- **Serialisasi Data:** `serde` + `serde_json` untuk file manifest `.blanc`, adegan `.bscene`, dan cerita `.bstory`.
- **Audio Engine:** `rodio` (cross-platform playback BGM dan SFX).
- **Native File Dialog:** `rfd` (Rust File Dialog untuk folder dan file picker native OS).
- **Image Decoding & Thumbnails:** `image` crate (PNG, JPG, WebP).
- **Pengelolaan Proyek:** Berorientasi pada penyimpanan lokal murni tanpa ketergantungan cloud.
- **No-Code Enforcement:** Seluruh konfigurasi dilakukan melalui antarmuka pengguna grafis tanpa scripting.

### Analytics & Tracking

Blanc Engine tidak memerlukan analytics berbasis cloud pada MVP.

Pengukuran awal dapat dilakukan melalui pengujian manual dan laporan bug dari pengguna.

#### Events to Track

| Event Name | Trigger | Properties | Purpose |
|---|---|---|---|
| `project_created` | Proyek berhasil dibuat | Project type, result | Memvalidasi pembuatan proyek |
| `asset_imported` | Aset berhasil diimpor | Asset category, result | Memvalidasi pengelolaan aset |
| `story_saved` | Cerita disimpan | Result | Memvalidasi penyimpanan cerita |
| `scene_created` | Scene dibuat | Scene count | Mengukur penggunaan scene editor |
| `action_block_added` | Block ditambahkan | Block type | Memahami penggunaan block |
| `preview_started` | Preview dijalankan | Scene ID, result | Memvalidasi runtime |
| `project_saved` | Proyek disimpan | Result | Memvalidasi persistence |

Pencatatan event pada tahap awal dapat dilakukan secara lokal atau melalui pengujian. Pengiriman data penggunaan ke layanan eksternal bukan bagian dari MVP.

---

## Go-to-Market Strategy

### Launch Plan

#### Beta/Alpha Testing

- **Alpha Testing:** Dilakukan oleh pengembang untuk memvalidasi fungsi inti.
- **Closed Beta:** Melibatkan sejumlah kreator visual novel atau pengguna yang tertarik membuat visual novel.
- **Feedback Loop:** Mengumpulkan laporan bug, kendala workflow, dan masukan terkait pengalaman penggunaan.

#### Phased Rollout

1. **Prototype:** Validasi alur utama aplikasi dan block-based editor.
2. **MVP:** Rilis fitur dasar pembuatan visual novel.
3. **Public Beta:** Membuka akses kepada pengguna umum untuk pengujian.
4. **Stable Release:** Merilis versi stabil setelah masalah utama diperbaiki.

#### Launch Date

TBD.

#### Launch Channels

- GitHub Repository.
- GitHub Releases.
- Komunitas pengembang game.
- Komunitas visual novel.
- Platform media sosial.
- Forum open-source dan game development.

### Marketing & Positioning

**Value Proposition:**

"Blanc Engine helps creators turn written stories into playable visual novels without writing code."

#### Key Messages

- **For Writers:** Ubah cerita yang sudah ditulis menjadi visual novel.
- **For Beginners:** Buat visual novel tanpa harus belajar scripting.
- **For Independent Developers:** Kelola aset dan scene melalui satu workflow visual.
- **For Open-Source Community:** Gunakan, pelajari, dan kontribusikan pengembangan Blanc Engine.

#### Marketing Channels

- GitHub.
- Reddit dan komunitas game development.
- Komunitas visual novel.
- Threads dan platform media sosial lainnya.
- Video demonstrasi penggunaan.
- Artikel pengembangan dan dokumentasi.

### Sales Enablement (if B2B)

Tidak berlaku untuk model produk awal karena Blanc Engine direncanakan sebagai proyek open-source, bukan produk B2B.

- **Sales Materials:** Tidak diperlukan.
- **Pricing/Packaging:** TBD. Tidak ada model harga yang ditetapkan.
- **Training Required:** Tidak berlaku.

### Support & Documentation

- **User Documentation:** Panduan instalasi, pembuatan proyek, pengelolaan aset, dan penyusunan scene.
- **Developer Documentation:** Dokumentasi arsitektur, struktur proyek, dan kontribusi kode.
- **Tutorial:** Tutorial pembuatan visual novel pertama.
- **Issue Tracking:** GitHub Issues.
- **Community Support:** TBD, bergantung pada perkembangan komunitas.

---

## Dependencies & Risks

### Dependencies

| Dependency | Owner | Status | Impact if Delayed | Mitigation |
|---|---|---|---|---|
| Rust Toolchain | Engineering | Planned | Pengembangan aplikasi tidak dapat dimulai | Menyiapkan toolchain sejak awal |
| Iced Framework | Engineering | Planned | Pengembangan UI terhambat | Menggunakan fitur framework yang stabil |
| Asset Loading | Engineering | TBD | Pengelolaan aset terganggu | Memvalidasi pendekatan pemuatan aset lebih awal |
| Audio Playback | Engineering | TBD | BGM dan SFX tidak dapat dijalankan | Membuat proof of concept audio |
| Runtime Rendering | Engineering | TBD | Preview visual novel tidak dapat berjalan | Mengembangkan runtime secara bertahap |
| Project Persistence | Engineering | Planned | Proyek tidak dapat disimpan dengan aman | Menentukan format data dan pengujian save/load |
| User Feedback | Product Owner | Pending | Kebutuhan pengguna belum tervalidasi | Mengadakan pengujian dengan calon pengguna |

### Risks & Mitigation

#### High Risk

**1. Kompleksitas Runtime dan Rendering**

- **Probability:** Medium
- **Impact:** High
- **Mitigation:** Memisahkan runtime dari UI editor dan membuat proof of concept rendering sejak tahap awal.
- **Owner:** Engineering.

**2. Scope Creep**

- **Probability:** High
- **Impact:** High
- **Mitigation:** Memprioritaskan P0 dan menunda fitur kompleks seperti branching, variables, dan plugin system.
- **Owner:** Product Owner.

**3. Kesulitan Menjaga Workflow No-Code**

- **Probability:** Medium
- **Impact:** High
- **Mitigation:** Memastikan seluruh fitur inti memiliki konfigurasi visual dan tidak membutuhkan scripting.
- **Owner:** Product Owner & Engineering.

#### Medium Risk

**1. Format Aset yang Beragam**

- **Probability:** Medium
- **Impact:** Medium
- **Mitigation:** Menentukan daftar format yang didukung dan melakukan validasi file saat impor.
- **Owner:** Engineering.

**2. Performa pada Proyek Besar**

- **Probability:** Medium
- **Impact:** Medium
- **Mitigation:** Mengoptimalkan pemuatan aset dan pengelolaan data scene.
- **Owner:** Engineering.

**3. Kurangnya Pengujian dari Pengguna Nyata**

- **Probability:** Medium
- **Impact:** Medium
- **Mitigation:** Mengadakan alpha dan beta testing dengan calon kreator visual novel.
- **Owner:** Product Owner.

### Open Questions & Decided Items

- [x] **Format data proyek:** Disepakati menggunakan file manifest `<NamaProject>.blanc` dan konfigurasi `Settings/system_screens.bconfig` (berbasis format serialisasi JSON).
- [x] **Arsitektur layar non-gameplay:** Disepakati menggunakan pendekatan *Built-in System Screens* otomatis oleh Runtime (Main Menu, Settings, Save/Load, In-Game Pause) dengan konfigurasi visual di Project Settings. Layar sekunder seperti Backlog, Splash Screen, Credits masuk di P1; Gallery & Music Room di P2.
- [x] **Lisensi open-source:** Disepakati **GNU General Public License v3 (GPLv3)** dengan perlindungan copyleft ketat untuk engine, sementara karya cerita/game buatan pengguna tetap hak milik eksklusif pembuatnya.
- [x] **Format file cerita untuk impor:** Disepakati Plain Text (`.txt`), Markdown (`.md`), dan format naskah skenario **Fountain (`.fountain`)** pada MVP; Twine (`.twee`) dan CSV pada P1.
- [x] **Arsitektur Preview Runtime:** Disepakati diluncurkan sebagai **Child Process** terpisah (`crates/blanc_runtime`) demi stabilitas editor dan isolasi memori.
- [x] **Bahasa Antarmuka (UI):** Disepakati menggunakan Bahasa Inggris standar industri untuk MVP dengan arsitektur lokalisasi untuk multi-bahasa di masa mendatang.
- [ ] Sistem operasi apa saja yang menjadi target rilis pertama?
- [ ] Platform apa yang akan didukung untuk export visual novel?
- [ ] Apakah Character dan Character Expression akan menjadi entitas terpisah dalam asset management?
- [ ] Bagaimana mekanisme distribusi runtime hasil export?
- [ ] Bagaimana strategi kompatibilitas proyek untuk versi Blanc Engine berikutnya?

---

## Timeline & Milestones

### Development Phases

Tanggal dan durasi final akan ditetapkan setelah estimasi teknis.

| Phase | Timeline | Key Deliverables | Owner |
|---|---|---|---|
| **Discovery** | TBD | Validasi kebutuhan, scope MVP, dan riset kompetitor | Product Owner |
| **Technical Design** | TBD | Arsitektur, data model, dan proof of concept | Engineering |
| **UI/UX Design** | TBD | Wireframe dan desain antarmuka | Design |
| **Development Sprint 1** | TBD | Project management dan guided workflow | Engineering |
| **Development Sprint 2** | TBD | Asset management dan story management | Engineering |
| **Development Sprint 3** | TBD | Block-based scene editor dan action configuration | Engineering |
| **Development Sprint 4** | TBD | Runtime, rendering, dan audio dasar | Engineering |
| **QA & Testing** | TBD | Pengujian integrasi, persistence, dan runtime | QA |
| **Alpha Release** | TBD | Build awal untuk pengujian internal | Engineering |
| **Beta Release** | TBD | Build untuk pengujian pengguna | Product Owner |
| **Stable Release** | TBD | Rilis MVP yang siap digunakan | Project Team |
| **Post-Launch** | TBD | Perbaikan, optimasi, dan pengembangan fitur lanjutan | Project Team |

### Key Milestones

- **M1 — Architecture Approved:** Fondasi arsitektur dan struktur data disepakati.
- **M2 — Project Workflow Complete:** Pengguna dapat membuat dan membuka proyek.
- **M3 — Asset & Story Management Complete:** Aset dan cerita dapat dikelola.
- **M4 — Scene Editor Complete:** Pengguna dapat menyusun scene melalui action block.
- **M5 — Runtime Prototype Complete:** Scene dapat dijalankan.
- **M6 — MVP Feature Complete:** Fitur P0 selesai dan terintegrasi.
- **M7 — Public Beta:** Blanc Engine tersedia untuk pengujian pengguna.
- **M8 — Stable Release:** Versi stabil pertama dirilis.

---

## Resources & Team

### Core Team

Struktur tim dapat disesuaikan dengan skala proyek.

- **Product Manager:** TBD.
- **Engineering Lead:** TBD.
- **Software Engineer:** TBD.
- **UI/UX Designer:** TBD.
- **QA Tester:** TBD.
- **Documentation & Community:** TBD.

Untuk tahap awal, beberapa tanggung jawab dapat dikerjakan oleh satu orang.

### Budget (if applicable)

Blanc Engine direncanakan sebagai proyek open-source dengan prioritas penggunaan teknologi dan tools yang dapat digunakan tanpa biaya lisensi komersial.

- **Development:** TBD.
- **Design:** TBD.
- **Marketing:** TBD.
- **Third-party Services:** TBD.
- **Total:** TBD.

---

## Post-Launch Plan

### Success Criteria Check-in Schedule

- **Week 1:**
  - Memeriksa laporan bug.
  - Memastikan aplikasi dapat dijalankan.
  - Memeriksa proses pembuatan dan pembukaan proyek.
  - Mengumpulkan kendala penggunaan pertama.

- **Week 4:**
  - Mengevaluasi workflow pengguna.
  - Memeriksa keberhasilan pengelolaan aset dan pembuatan scene.
  - Mengevaluasi performa aplikasi.
  - Mengelompokkan masukan berdasarkan prioritas.

- **Week 12:**
  - Mengevaluasi stabilitas MVP.
  - Memeriksa kebutuhan fitur lanjutan.
  - Menentukan prioritas pengembangan versi berikutnya.
  - Mengevaluasi potensi kontribusi komunitas.

### Iteration Plan

- **Feedback Collection:** GitHub Issues, diskusi komunitas, dan pengujian langsung.
- **Update Cadence:** TBD, disesuaikan dengan kapasitas pengembangan.
- **Bug Fixes:** Masalah kritis akan diprioritaskan.
- **Feature Development:** Fitur baru mengikuti prioritas P1 dan P2.
- **Sunset Plan:** Belum diperlukan karena produk masih berada pada tahap awal pengembangan.

### Learning Goals

1. Mengetahui apakah workflow no-code benar-benar memudahkan pembuatan visual novel.
2. Mengetahui apakah block-based editor mudah dipahami oleh pengguna baru.
3. Mengetahui jenis fitur yang paling dibutuhkan kreator visual novel.
4. Mengidentifikasi hambatan teknis pada runtime dan rendering.
5. Memahami kebutuhan distribusi visual novel hasil export.
6. Mengetahui potensi komunitas dan kontribusi open-source.

Hasil pembelajaran akan digunakan untuk menentukan arah pengembangan Blanc Engine setelah MVP.

---

## Appendix

### References

- Software Requirements Specification (SRS) — Blanc Engine.
- Product Requirements Document (PRD) Template.
- Official Rust Documentation.
- Official Iced Documentation.
- Ren'Py Documentation.
- Visual Novel Maker.
- TyranoBuilder.

### Changelog

| Date | Version | Changes | Author |
|---|---|---|---|
| 12 November 2026 | 0.1 | Initial PRD draft | Product Owner |

### Stakeholder Approvals

- [ ] Engineering Lead — TBD — TBD
- [ ] Design Lead — TBD — TBD
- [ ] QA Lead — TBD — TBD
- [ ] Product Owner — TBD — TBD

---

**End of Document**