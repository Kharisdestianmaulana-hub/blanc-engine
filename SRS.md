# Software Requirements Specification (SRS)
# Blanc Engine

**Version:** 0.1  
**Status:** Initial Draft  
**Product Name:** Blanc Engine  
**Pronunciation:** Blang Engine  
**Product Type:** No-Code Visual Novel Creation Engine  
**Programming Language:** Rust  
**UI Framework:** Iced  
**Development Model:** Open Source  
**License:** GNU General Public License v3 (GPLv3)  
**Prepared:** 12 November 2026

---

# Revision History

| Version | Date | Description | Status |
|---|---|---|---|
| 0.1 | 12 November 2026 | Initial SRS document | Draft |

# 1. Introduction

## 1.1 Document Purpose

Dokumen Software Requirements Specification (SRS) ini bertujuan untuk mendefinisikan kebutuhan fungsional dan non-fungsional Blanc Engine, sebuah game engine khusus pembuatan visual novel berbasis no-code.

Dokumen ini menjadi acuan utama bagi proses perancangan, pengembangan, pengujian, dan pemeliharaan Blanc Engine. Dokumen ditujukan kepada pengembang, kontributor open-source, UI/UX designer, dan pihak lain yang terlibat dalam pengembangan produk.

SRS ini mendefinisikan apa yang harus dapat dilakukan oleh sistem, bukan menjelaskan seluruh detail implementasi internalnya.

## 1.2 Product Scope

Blanc Engine adalah aplikasi desktop open-source yang memungkinkan pengguna membuat visual novel tanpa memerlukan kemampuan pemrograman.

Pengguna dapat mengelola aset, memasukkan cerita, menyusun adegan menggunakan sistem block-based, mengatur dialog dan elemen visual, melakukan preview, serta menghasilkan game visual novel yang dapat dimainkan secara mandiri.

Blanc Engine dirancang untuk menyederhanakan proses pembuatan visual novel dengan menyediakan workflow bertahap dan editor visual yang mudah digunakan.

### In Scope

Fitur yang termasuk dalam cakupan pengembangan Blanc Engine:

1. Project creation dan project management.
2. Asset Manager untuk mengelola seluruh aset visual novel.
3. Story Manager untuk mengelola cerita, dialog, dan narasi.
4. Story Import untuk memasukkan cerita yang telah dibuat sebelumnya.
5. Scene Editor berbasis vertical block.
6. Action Block untuk mengatur elemen di dalam scene.
7. Character dan expression management.
8. Background, BGM, dan SFX management.
9. Dialogue dan narration system.
10. Scene transition dan scene navigation.
11. Player choice dan branching system.
12. Visual novel preview dan testing.
13. Project validation.
14. Save, autosave, dan recovery.
15. Game build dan export.
16. Standalone visual novel runtime.

### Out of Scope

Hal-hal berikut tidak menjadi fokus utama pengembangan awal:

1. Game development berbasis 3D.
2. General-purpose game engine.
3. Pembuatan game menggunakan pemrograman secara manual.
4. Multiplayer dan fitur online multiplayer.
5. Real-time collaborative editing.
6. Integrated AI story generation.
7. Integrated AI image generation.
8. Marketplace aset bawaan.
9. Advanced game physics.
10. General-purpose scripting environment.

Fitur di luar cakupan awal dapat dipertimbangkan pada pengembangan versi berikutnya.

## 1.3 Definitions, Acronyms, and Abbreviations

| Istilah | Definisi |
|---|---|
| Blanc Engine | Game engine khusus pembuatan visual novel tanpa coding. |
| No-Code | Pendekatan pembuatan aplikasi atau game tanpa menulis kode program. |
| Project | Kumpulan data, aset, konfigurasi, dan scene yang membentuk satu visual novel. |
| `.blanc` File | File manifest utama project (`<NamaProject>.blanc`) yang menyimpan metadata, mapping aset, dan konfigurasi project dalam format JSON. |
| `.bscene` File | Format file adegan (`<nama_adegan>.bscene`) berbasis JSON yang menyimpan susunan Action Blocks di dalam folder chapter. |
| `.bstory` File | Format file naskah cerita internal (`<chapter_name>.bstory`) yang dikelola di dalam folder Story. |
| Fountain | Format penulisan naskah/skenario terbuka berbasis teks polos (`.fountain`) yang didukung untuk impor cerita secara langsung. |
| Asset | File yang digunakan dalam visual novel, seperti gambar, audio, dan font. |
| Asset ID | Pengenal unik untuk setiap aset guna menjaga integritas referensi antar-scene. |
| Asset Manager | Modul untuk mengelola aset project. |
| Story | Kumpulan cerita yang berisi dialog, narasi, dan elemen cerita lainnya. |
| Chapter | Bagian atau bab dalam sebuah cerita. |
| Scene | Satu kesatuan adegan dalam visual novel. |
| Story Scene | Adegan naratif/gameplay yang disusun secara sekuensial menggunakan Action Blocks vertikal. |
| System Screen | Layar sistemik non-gameplay (Main Menu, Settings, Save/Load, Pause Menu) yang disediakan otomatis oleh runtime. |
| Action Block | Elemen visual yang merepresentasikan sebuah aksi di dalam scene. |
| Scene Editor | Modul untuk menyusun dan mengatur scene cerita. |
| Dialogue | Percakapan antara karakter atau pemain. |
| Narration | Narasi yang disampaikan tanpa harus berasal dari karakter tertentu. |
| BGM | Background Music, musik latar. |
| SFX | Sound Effects, efek suara. |
| Runtime | Sistem yang menjalankan visual novel hasil pembuatan engine. |
| Preview | Pratinjau visual novel sebelum dilakukan export. |
| Build | Proses menghasilkan aplikasi visual novel yang dapat dimainkan. |
| Branching | Sistem percabangan cerita berdasarkan pilihan pemain atau kondisi tertentu. |
| Inspector | Panel untuk mengatur properti elemen yang dipilih. |
| Autosave | Penyimpanan project secara otomatis. |
| SRS | Software Requirements Specification. |
| UI | User Interface. |
| UX | User Experience. |

## 1.4 References

| Referensi | Keterangan |
|---|---|
| SRS Template | Struktur dokumen Software Requirements Specification yang digunakan sebagai acuan penyusunan dokumen ini. |
| Rust Documentation | Dokumentasi resmi bahasa pemrograman Rust. |
| Iced Documentation | Dokumentasi resmi framework Iced untuk pengembangan aplikasi GUI menggunakan Rust. |

Referensi dokumentasi teknis:

- Rust: https://www.rust-lang.org/
- Iced: https://iced.rs/

## 1.5 Document Overview

Dokumen ini dibagi menjadi lima bagian utama.

- **Bab 1 – Introduction:** Menjelaskan tujuan, ruang lingkup, dan terminologi Blanc Engine.
- **Bab 2 – Product Overview:** Menjelaskan gambaran produk, fungsi utama, batasan, dan karakteristik pengguna.
- **Bab 3 – Requirements:** Mendefinisikan kebutuhan fungsional, antarmuka, performa, keamanan, dan implementasi.
- **Bab 4 – Verification:** Menjelaskan metode pengujian untuk memastikan seluruh requirement terpenuhi.
- **Bab 5 – Appendixes:** Berisi informasi tambahan yang mendukung spesifikasi produk.

Requirement menggunakan istilah berikut:

- **MUST:** Wajib dipenuhi.
- **SHOULD:** Sangat direkomendasikan untuk dipenuhi.
- **MAY:** Bersifat opsional.

---

# 2. Product Overview

## 2.1 Product Perspective

Blanc Engine merupakan produk perangkat lunak baru yang dikembangkan sebagai game engine khusus visual novel.

Berbeda dari game engine general-purpose yang menyediakan berbagai sistem pengembangan game, Blanc Engine berfokus pada satu jenis game, yaitu visual novel.

Fokus utama produk adalah memberikan pengalaman pembuatan game yang dapat digunakan oleh pengguna tanpa latar belakang programming.

Blanc Engine memiliki beberapa subsistem utama:

1. **Desktop Application:** Antarmuka utama untuk seluruh aktivitas pengguna.
2. **Project Management:** Mengatur pembuatan, pembukaan, dan penyimpanan project.
3. **Asset Management:** Mengelola aset visual dan audio.
4. **Story Management:** Mengelola cerita dan elemen dialog.
5. **Scene Management:** Mengatur struktur dan hubungan antar-scene.
6. **Visual Novel Runtime:** Menjalankan visual novel berdasarkan data project.
7. **Build and Export:** Menghasilkan game yang dapat dimainkan secara mandiri.

Secara konseptual, arsitektur produk:

```text
                 BLANC ENGINE
                       |
          +------------+------------+
          |                         |
     EDITOR SYSTEM              PROJECT DATA
          |                         |
    +-----+------+            +-----+------+
    |     |      |            |            |
  Asset Story  Scene       Assets       Scenes
 Manager Manager Editor    Metadata     Data
          |                         |
          +------------+------------+
                       |
                 PREVIEW SYSTEM
                       |
                RUNTIME ENGINE
                       |
                 BUILD SYSTEM
                       |
                STANDALONE GAME
```

Editor dan runtime dirancang sebagai bagian yang terpisah secara tanggung jawab. Runtime bertugas menjalankan visual novel, sedangkan editor bertugas membuat dan mengelola konten visual novel.

Dalam arsitektur layarnya, Blanc Engine membedakan dua jenis layar:
1. **Story Scenes (Layar Gameplay/Cerita):** Adegan interaktif yang disusun pengguna menggunakan susunan Action Block vertikal (Background, Character, Dialogue, BGM, SFX, Transition, Choice).
2. **Built-in System Screens (Layar Sistemik Non-Gameplay):** Layar fungsionalitas yang otomatis disediakan oleh Runtime tanpa perlu disusun manual oleh pengguna menggunakan blok logika. Ini mencakup Main Menu (Title Screen), Settings/Preferences Screen, Save & Load Screen, dan In-Game Pause Menu. Pengguna cukup mengonfigurasi aset visual (background, musik, logo, font) melalui panel Project Settings.

## 2.2 Product Functions

Blanc Engine menyediakan fungsi utama berikut:

### 1. Project Management

Membuat project baru, menentukan lokasi penyimpanan, mengelola file manifest `<NamaProject>.blanc`, membuka project, dan mengelola konfigurasi project.

### 2. Guided Workflow

Menyediakan proses pembuatan visual novel secara bertahap. Pengguna dapat berpindah antarhalaman berdasarkan status penyelesaian setiap tahapan.

### 3. Asset Management

Mengimpor, mengategorikan, melihat, mencari, dan mengelola aset yang digunakan dalam visual novel.

### 4. Story Management

Mengimpor cerita yang sudah dibuat pengguna serta mengelola dialog, narasi, chapter, dan karakter pembicara.

### 5. Scene Editor

Menyusun adegan cerita menggunakan Action Block yang dapat ditambahkan, dihapus, diubah urutannya, dan dikonfigurasi tanpa coding.

### 6. System Screens Management

Mengonfigurasi tampilan dan audio untuk layar sistem bawaan (Main Menu, Settings, Save/Load, In-Game Pause Menu) tanpa mengharuskan pengguna membuat tombol atau logika menu secara manual.

### 7. Visual Novel Runtime

Menjalankan alur permainan visual novel (baik System Screens maupun Story Scenes), menampilkan background dan karakter, memainkan dialog, mengelola audio, serta menangani pilihan pemain dan state game.

### 8. Preview and Testing

Memungkinkan pengguna menguji hasil visual novel pada jendela terpisah sebelum melakukan build.

### 9. Build and Export

Menghasilkan visual novel menjadi game yang dapat dimainkan tanpa membuka editor Blanc Engine.

## 2.3 Product Constraints

Blanc Engine memiliki batasan dan ketentuan berikut:

1. Aplikasi harus dikembangkan menggunakan bahasa pemrograman Rust dengan struktur arsitektur **Cargo Workspace** modular (`crates/blanc_core`, `crates/blanc_editor`, `crates/blanc_runtime`).
2. Framework GUI yang digunakan adalah Iced.
3. Aplikasi berorientasi pada desktop dengan resolusi kanvas dasar default **`1920 × 1080` (16:9 Full HD)** serta mendukung proportional scaling dan letterboxing pada layar berbeda.
4. Seluruh proses pembuatan visual novel harus dapat dilakukan tanpa menulis kode.
5. Sistem penyusunan scene cerita menggunakan susunan vertikal Action Block dengan format file adegan **`.bscene`** yang diorganisasi di dalam subfolder chapter.
6. Project dikelola melalui file manifest utama **`<NamaProject>.blanc`** (berbasis format JSON) dan harus dapat disimpan serta dibuka kembali secara konsisten.
7. Aset pengguna dikelola menggunakan sistem **Asset ID unik** untuk mencegah kerusakan referensi antar-scene saat file di-rename di disk.
8. Runtime harus dapat membaca data visual novel yang dihasilkan editor dan menyediakan System Screens bawaan.
9. Pengembangan harus memperhatikan penggunaan memori dan penyimpanan.
10. Produk dikembangkan sebagai perangkat lunak open-source.
11. Fitur utama tidak boleh mensyaratkan koneksi internet.
12. Pengguna harus memiliki hak akses terhadap folder tempat project disimpan.

Detail sistem operasi target, lisensi open-source, format distribusi, dan target platform export masih perlu ditetapkan dalam keputusan teknis berikutnya.

## 2.4 User Characteristics

Blanc Engine dirancang untuk beberapa kategori pengguna.

### 1. Beginner Creator

Pengguna yang belum memiliki pengalaman dalam pengembangan game.

Karakteristik:

- Tidak memiliki kemampuan programming.
- Memiliki ide cerita visual novel.
- Membutuhkan panduan dalam proses pembuatan game.
- Mengutamakan kemudahan penggunaan.

### 2. Story Writer

Pengguna yang berfokus pada pembuatan cerita.

Karakteristik:

- Memiliki cerita yang sudah disiapkan.
- Membutuhkan sistem pengelolaan dialog dan narasi.
- Ingin mengubah cerita menjadi pengalaman interaktif.

### 3. Artist and Creative Designer

Pengguna yang memiliki aset visual dan audio.

Karakteristik:

- Memiliki ilustrasi karakter, background, musik, atau efek suara.
- Membutuhkan pengelolaan aset yang terstruktur.
- Ingin mengatur tampilan visual novel tanpa coding.

### 4. Indie Game Developer

Pengguna yang sudah memiliki pengalaman membuat game.

Karakteristik:

- Menginginkan proses pembuatan visual novel yang lebih efisien.
- Membutuhkan pengaturan scene yang fleksibel.
- Menginginkan kontrol lebih terhadap tampilan dan alur cerita.

### 5. Open-Source Contributor

Pengembang yang ingin berkontribusi terhadap pengembangan Blanc Engine.

Karakteristik:

- Memahami pemrograman dan pengembangan perangkat lunak.
- Membantu pengembangan fitur, perbaikan bug, dan dokumentasi.
- Mengikuti standar kontribusi project open-source.

## 2.5 Assumptions and Dependencies

### Assumptions

1. Pengguna memiliki perangkat desktop yang memenuhi kebutuhan minimum aplikasi.
2. Pengguna memiliki aset visual atau audio sendiri, atau memperoleh aset dari sumber yang sah.
3. Pengguna memiliki hak penggunaan terhadap seluruh aset yang dimasukkan ke project.
4. Pengguna dapat mengakses folder penyimpanan project.
5. Pengguna memahami dasar penyusunan cerita, dialog, atau narasi.
6. Pengguna tidak diwajibkan memahami bahasa pemrograman.

### Dependencies

1. Rust sebagai bahasa pemrograman utama.
2. Iced sebagai framework antarmuka desktop.
3. Library pendukung untuk pemrosesan gambar dan audio.
4. Sistem filesystem untuk mengelola project dan aset.
5. Komponen rendering untuk menampilkan visual novel.
6. Komponen audio untuk memainkan BGM dan SFX.

Pemilihan library pendukung secara spesifik akan ditentukan dalam dokumen arsitektur teknis.

## 2.6 Apportioning of Requirements

Persyaratan dibagi menjadi beberapa tahap pengembangan.

| Modul | MVP | Pengembangan Lanjutan |
|---|---|---|
| Project Management (`.blanc`) | Ya | Pengembangan template dan project recovery |
| Guided Workflow | Ya | Workflow customization |
| Asset Manager (Asset ID System) | Ya | Tagging dan advanced filtering |
| Story Manager (`.bstory`) | Ya | Advanced story tools |
| Story Import | Ya (TXT, Markdown, Fountain) | Twine (.twee), CSV, dan format naskah lanjutan |
| Scene Editor (Story Scenes - `.bscene`) | Ya | Block tambahan dan fitur lanjutan |
| System Screens (Main Menu, Settings, Save/Load, Pause Menu) | Ya (Built-in runtime) | Kustomisasi layout UI lanjutan |
| Dialogue System | Ya | Advanced dialogue behavior |
| Character System | Ya (Full Sprite per Ekspresi, Posisi Left/Center/Right) | Layered sprite & animasi lanjutan |
| Audio System | Ya | Audio mixing lanjutan |
| Preview Player (1920x1080) | Ya | Advanced debugging |
| Branching System | Dasar | Conditional branching kompleks |
| Save/Load Runtime | Dasar | Multiple save slots |
| Non-Gameplay P1 (Backlog, Splash Screen, Credits Roll) | Tidak | Ya (P1 - Pasca-MVP) |
| Non-Gameplay P2 (CG Gallery, Music Room, Scene Replay) | Tidak | Ya (P2 - Ekstra) |
| Build/Export | Ya | Multi-platform distribution |
| Plugin System | Tidak | Ya |
| Marketplace | Tidak | Opsional |

---

# 3. Requirements

## 3.1 External Interfaces

### 3.1.1 User Interfaces

Blanc Engine harus menyediakan antarmuka desktop yang konsisten, mudah dipahami, dan mendukung proses pembuatan visual novel tanpa coding.

Antarmuka utama terdiri dari:

1. Welcome Screen.
2. Project Creation.
3. Project Dashboard.
4. Asset Manager.
5. Story Manager.
6. Scene Editor.
7. Preview Player.
8. Build and Export.
9. Project Settings.

### A. Welcome Screen

Halaman awal aplikasi yang menyediakan akses untuk:

- Membuat project baru.
- Membuka project yang sudah ada.
- Melihat project terbaru.
- Mengakses pengaturan aplikasi.

### B. Project Creation

Halaman untuk menentukan informasi project baru.

Informasi yang diperlukan:

- Project name.
- Project location.
- Project configuration.

Pengguna harus memilih folder penyimpanan sebelum project dibuat.

### C. Project Dashboard

Halaman utama setelah project berhasil dibuat.

Dashboard menyediakan:

- Informasi project.
- Status penyelesaian workflow.
- Navigasi antarbagian.
- Ringkasan aset.
- Ringkasan chapter dan scene.
- Akses preview dan build.

### D. Asset Manager

Antarmuka pengelolaan aset menggunakan kategori:

- Backgrounds.
- Characters.
- Character Expressions.
- BGM.
- SFX.
- Fonts.
- UI Assets.
- Other Assets.

Pengguna dapat melakukan import, preview, rename, delete, dan pengelompokan aset.

### E. Story Manager

Antarmuka untuk memasukkan dan mengelola cerita.

Pengguna dapat:

- Mengimpor cerita.
- Membuat chapter.
- Mengelola dialog.
- Mengelola narasi.
- Mengatur nama pembicara.
- Mengubah urutan elemen cerita.

### F. Scene Editor

Scene Editor merupakan antarmuka utama dalam pembuatan visual novel.

Antarmuka menggunakan sistem block-based dengan susunan vertikal.

Komponen antarmuka:

- Scene Navigator.
- Scene Canvas.
- Action Block.
- Block Inspector.
- Preview Panel.
- Add Block Menu.

### G. Preview Player

Menampilkan hasil visual novel dalam bentuk yang menyerupai pengalaman bermain game sebenarnya.

### H. Build and Export

Menyediakan pengaturan untuk menghasilkan game dari project Blanc Engine.

### Navigasi

Sistem navigasi harus menyediakan kemampuan untuk berpindah antarhalaman dengan ketentuan:

1. Pengguna dapat kembali ke halaman sebelumnya kapan saja.
2. Halaman berikutnya hanya dapat diakses apabila persyaratan halaman sebelumnya terpenuhi.
3. Halaman yang telah selesai dapat dibuka kembali untuk diedit.
4. Jika perubahan menyebabkan tahapan sebelumnya tidak lagi valid, sistem harus memperbarui status penyelesaian.
5. Sistem harus menunjukkan alasan apabila pengguna belum dapat melanjutkan.

### 3.1.2 Hardware Interfaces

Blanc Engine merupakan aplikasi desktop yang berinteraksi dengan perangkat keras melalui sistem operasi.

Antarmuka perangkat keras meliputi:

- CPU untuk menjalankan aplikasi dan proses rendering.
- RAM untuk penyimpanan data sementara.
- Storage untuk project dan aset.
- Display untuk menampilkan antarmuka dan preview.
- Audio output untuk menguji BGM dan SFX.

Dukungan hardware spesifik akan ditetapkan dalam spesifikasi minimum sistem.

### 3.1.3 Software Interfaces

| Software | Fungsi |
|---|---|
| Rust | Bahasa pemrograman utama |
| Iced | Framework antarmuka |
| Filesystem | Penyimpanan dan pengelolaan project |
| Image Processing Library | Pemrosesan dan preview gambar |
| Audio Library | Pemutaran dan pengelolaan audio |
| Rendering Component | Menampilkan elemen visual novel |
| Runtime Component | Menjalankan data visual novel |
| Build System | Menghasilkan game standalone |

---

# 3.2 Functional Requirements

## 3.2.1 Project Management

### REQ-FUNC-001 — Create New Project

- **ID:** REQ-FUNC-001
- **Title:** Membuat project baru.
- **Statement:** Sistem harus menyediakan fitur pembuatan project visual novel baru dengan file manifest `<NamaProject>.blanc`.
- **Rationale:** Pengguna membutuhkan wadah untuk menyimpan seluruh data visual novel dengan identitas file yang jelas.
- **Acceptance Criteria:**
  - Pengguna dapat memasukkan nama project.
  - Pengguna dapat memilih folder penyimpanan.
  - Sistem memvalidasi nama dan lokasi project.
  - Sistem membuat struktur folder project secara otomatis.
  - Sistem menghasilkan file manifest `<NamaProject>.blanc` (berformat JSON) yang berisi metadata dan konfigurasi awal project.
  - Sistem membuka project setelah proses pembuatan berhasil.
- **Verification Method:** Test, Demonstration.

### REQ-FUNC-002 — Project Location

- **ID:** REQ-FUNC-002
- **Title:** Menentukan lokasi penyimpanan project.
- **Statement:** Sistem harus memungkinkan pengguna memilih folder tempat project dan asetnya disimpan.
- **Rationale:** Pengguna harus memiliki kontrol terhadap lokasi penyimpanan project.
- **Acceptance Criteria:**
  - Sistem menyediakan folder picker.
  - Sistem menampilkan lokasi yang dipilih.
  - Sistem memeriksa izin akses folder.
  - Sistem memberikan informasi apabila lokasi tidak dapat digunakan.
- **Verification Method:** Test.

### REQ-FUNC-003 — Open Existing Project

- **ID:** REQ-FUNC-003
- **Title:** Membuka project yang sudah ada.
- **Statement:** Sistem harus dapat membuka project Blanc Engine melalui pemilihan file `<NamaProject>.blanc` atau folder project.
- **Acceptance Criteria:**
  - Pengguna dapat memilih file `<NamaProject>.blanc` atau folder project.
  - Sistem membaca konfigurasi project dari file `.blanc`.
  - Sistem memvalidasi struktur project.
  - Sistem menampilkan pesan kesalahan jika project tidak valid atau versi format tidak didukung.
- **Verification Method:** Test.

### REQ-FUNC-004 — Save Project

- **ID:** REQ-FUNC-004
- **Title:** Menyimpan perubahan project.
- **Statement:** Sistem harus menyediakan fitur penyimpanan seluruh perubahan project.
- **Acceptance Criteria:**
  - Pengguna dapat menyimpan perubahan secara manual.
  - Perubahan pada asset, story, scene, dan pengaturan project tersimpan.
  - Sistem memberikan indikator status penyimpanan.
  - Sistem tidak menganggap penyimpanan berhasil sebelum proses selesai.
- **Verification Method:** Test.

### REQ-FUNC-005 — Autosave and Recovery

- **ID:** REQ-FUNC-005
- **Title:** Autosave dan pemulihan project.
- **Statement:** Sistem harus menyediakan mekanisme penyimpanan otomatis dan pemulihan perubahan yang belum tersimpan setelah aplikasi mengalami gangguan.
- **Acceptance Criteria:**
  - Sistem dapat melakukan autosave.
  - Sistem tidak membuat konflik antara autosave dan penyimpanan manual.
  - Sistem dapat mendeteksi data pemulihan yang tersedia.
  - Sistem memberikan pilihan pemulihan kepada pengguna.
- **Verification Method:** Test.

## 3.2.2 Guided Workflow

### REQ-FUNC-006 — Workflow Navigation

- **ID:** REQ-FUNC-006
- **Title:** Navigasi workflow bertahap.
- **Statement:** Sistem harus menyediakan navigasi bertahap untuk membantu pengguna menyelesaikan proses pembuatan visual novel.
- **Acceptance Criteria:**
  - Sistem menampilkan seluruh tahapan workflow.
  - Pengguna dapat melihat status setiap tahapan.
  - Pengguna dapat kembali ke tahapan sebelumnya.
  - Tahapan berikutnya hanya dapat dibuka jika persyaratan tahap sebelumnya terpenuhi.
  - Sistem memberikan informasi mengenai persyaratan yang belum dipenuhi.
- **Verification Method:** Test, Demonstration.

### REQ-FUNC-007 — Workflow Validation

- **ID:** REQ-FUNC-007
- **Title:** Validasi penyelesaian tahapan.
- **Statement:** Sistem harus memvalidasi setiap tahapan sebelum mengizinkan pengguna melanjutkan ke tahap berikutnya.
- **Acceptance Criteria:**
  - Sistem memeriksa persyaratan setiap tahap.
  - Status tahap diperbarui setelah perubahan dilakukan.
  - Tahap yang tidak lagi memenuhi persyaratan ditandai belum selesai.
  - Pengguna mendapatkan penjelasan mengenai masalah yang ditemukan.
- **Verification Method:** Test.

## 3.2.3 Asset Management

### REQ-FUNC-008 — Asset Import and Supported Formats

- **ID:** REQ-FUNC-008
- **Title:** Mengimpor aset dan format file yang didukung.
- **Statement:** Sistem harus memvalidasi dan mengizinkan impor file aset dari perangkat pengguna sesuai daftar format resmi yang didukung engine.
- **Acceptance Criteria:**
  - Pengguna dapat memilih satu atau beberapa file dari sistem berkas.
  - Sistem memvalidasi ekstensi dan tipe MIME file dengan batasan format resmi:
    - **Gambar:** PNG (`.png` — wajib untuk transparansi sprite), JPG/JPEG (`.jpg`, `.jpeg` — background), dan WebP (`.webp` — kompresi modern efisien).
    - **Audio:** OGG Vorbis (`.ogg` — standar visual novel untuk looping BGM mulus), WAV (`.wav` — SFX berlatensi rendah), dan MP3 (`.mp3`).
    - **Font:** TrueType Font (`.ttf`) dan OpenType Font (`.otf`).
  - Sistem mengimpor dan menyalin file ke subdirektori project yang sesuai di bawah folder `Assets/`.
  - Sistem memberikan pesan penolakan dan informasi yang jelas jika pengguna mengimpor format yang tidak didukung atau file yang rusak.
  - Sistem tidak mengubah atau merusak file asli di perangkat pengguna.
- **Verification Method:** Test.

### REQ-FUNC-009 — Asset Categorization

- **ID:** REQ-FUNC-009
- **Title:** Mengategorikan aset.
- **Statement:** Sistem harus menyediakan kategori untuk mengorganisasi aset visual novel.
- **Acceptance Criteria:**
  - Tersedia kategori Background.
  - Tersedia kategori Character.
  - Tersedia kategori Character Expression.
  - Tersedia kategori BGM.
  - Tersedia kategori SFX.
  - Tersedia kategori Fonts.
  - Tersedia kategori UI Assets.
  - Tersedia kategori Other Assets.
  - Pengguna dapat memindahkan aset antar kategori.
- **Verification Method:** Test.

### REQ-FUNC-010 — Asset Preview

- **ID:** REQ-FUNC-010
- **Title:** Pratinjau aset.
- **Statement:** Sistem harus menyediakan fitur preview untuk aset yang didukung.
- **Acceptance Criteria:**
  - Gambar dapat ditampilkan sebagai thumbnail atau preview.
  - Audio dapat diputar dan dihentikan.
  - Font dapat diperlihatkan melalui preview teks.
  - File yang tidak didukung menampilkan informasi yang sesuai.
- **Verification Method:** Test, Demonstration.

### REQ-FUNC-011 — Asset Organization and Asset ID Mapping

- **ID:** REQ-FUNC-011
- **Title:** Pengelolaan aset dan sistem Asset ID.
- **Statement:** Sistem harus memungkinkan pengguna mengelola aset yang telah dimasukkan ke project menggunakan pengenal unik (Asset ID).
- **Acceptance Criteria:**
  - Setiap aset diberikan identitas unik (Asset ID) saat diimpor.
  - Manifest `.blanc` mencatat pemetaan antara Asset ID dan lokasi file aset di disk.
  - Pengguna dapat mengganti nama aset (display name atau filename) tanpa merusak referensi pada Action Block yang menggunakannya.
  - Pengguna dapat menghapus aset.
  - Pengguna dapat mencari dan memfilter aset berdasarkan kategori.
  - Pengguna dapat melihat detail aset.
  - Sistem memberikan peringatan sebelum menghapus aset yang sedang digunakan di scene.
- **Verification Method:** Test.

### REQ-FUNC-012 — Asset Usage Tracking

- **ID:** REQ-FUNC-012
- **Title:** Pelacakan penggunaan aset.
- **Statement:** Sistem sebaiknya menyediakan informasi penggunaan aset di dalam scene berdasarkan Asset ID.
- **Acceptance Criteria:**
  - Sistem dapat mengidentifikasi aset yang digunakan berdasarkan Asset ID.
  - Sistem dapat menampilkan scene yang menggunakan aset.
  - Sistem dapat mengidentifikasi aset yang belum digunakan.
- **Verification Method:** Test.

## 3.2.4 Story Management

### REQ-FUNC-013 — Story Import

- **ID:** REQ-FUNC-013
- **Title:** Mengimpor cerita dari naskah eksternal.
- **Statement:** Sistem harus menyediakan kemampuan untuk memasukkan cerita dari naskah yang telah dibuat pengguna dalam format teks polos, Markdown, maupun format penulisan naskah standar industri.
- **Acceptance Criteria:**
  - Sistem mendukung impor file Plain Text (`.txt`).
  - Sistem mendukung impor file Markdown (`.md`).
  - Sistem mendukung impor file naskah skenario standar industri **Fountain (`.fountain`)** yang umum dihasilkan oleh Scrivener, Highland, Fade In, dan WriterDuet.
  - Parser otomatis membedakan heading adegan/chapter (`[Chapter]`, `[Scene]`, atau `EXT./INT.`), dialog berkarakter (`Nama: "Dialog"` atau format naskah Fountain `NAMA \n (ekspresi) \n Dialog`), dan narasi umum.
  - Sistem menampilkan hasil parsing kepada pengguna sebelum dikonversi menjadi elemen cerita atau adegan.
  - Sistem memberikan pesan kesalahan informatif jika format tidak dapat dibaca.
- **Verification Method:** Test.

### REQ-FUNC-014 — Story Editor and Internal Persistence

- **ID:** REQ-FUNC-014
- **Title:** Mengelola dan menyimpan cerita internal.
- **Statement:** Sistem harus menyediakan editor internal untuk membuat dan mengubah konten cerita yang disimpan dalam format file `.bstory` internal.
- **Acceptance Criteria:**
  - Konten cerita disimpan secara internal dalam file `.bstory` (format JSON) di dalam subfolder `Story/Chapters/`.
  - Pengguna dapat menambahkan cerita langsung melalui editor.
  - Pengguna dapat mengedit teks cerita.
  - Pengguna dapat menghapus elemen cerita.
  - Pengguna dapat mengatur urutan elemen cerita.
  - Perubahan tersimpan ke project.
- **Verification Method:** Test.

### REQ-FUNC-015 — Chapter Management

- **ID:** REQ-FUNC-015
- **Title:** Pengelolaan chapter.
- **Statement:** Sistem harus memungkinkan pengguna mengelompokkan cerita ke dalam chapter.
- **Acceptance Criteria:**
  - Pengguna dapat membuat chapter.
  - Pengguna dapat mengganti nama chapter.
  - Pengguna dapat menghapus chapter.
  - Pengguna dapat mengubah urutan chapter.
  - Setiap chapter dapat memiliki kumpulan elemen cerita.
- **Verification Method:** Test.

### REQ-FUNC-016 — Dialogue and Narration Management

- **ID:** REQ-FUNC-016
- **Title:** Pengelolaan dialog dan narasi.
- **Statement:** Sistem harus membedakan dialog karakter dengan narasi cerita.
- **Acceptance Criteria:**
  - Pengguna dapat membuat dialog.
  - Pengguna dapat menentukan pembicara.
  - Pengguna dapat membuat narasi tanpa pembicara.
  - Pengguna dapat mengubah isi dialog dan narasi.
  - Setiap elemen cerita memiliki identitas unik.
- **Verification Method:** Test.

### REQ-FUNC-017 — Story Element Assignment

- **ID:** REQ-FUNC-017
- **Title:** Menghubungkan elemen cerita dengan scene.
- **Statement:** Sistem harus memungkinkan elemen cerita yang telah dibuat dipilih dan digunakan di dalam scene.
- **Acceptance Criteria:**
  - Pengguna dapat memilih dialog dari Story Manager.
  - Pengguna dapat memilih narasi dari Story Manager.
  - Elemen cerita dapat digunakan dalam scene.
  - Perubahan pada elemen cerita dapat disinkronkan dengan scene yang menggunakannya.
  - Sistem dapat mengidentifikasi referensi elemen cerita yang tidak valid.
- **Verification Method:** Test.

## 3.2.5 Scene Management

### REQ-FUNC-018 — Scene Creation and Storage

- **ID:** REQ-FUNC-018
- **Title:** Membuat scene baru dan struktur file `.bscene`.
- **Statement:** Sistem harus memungkinkan pengguna membuat scene di dalam chapter yang disimpan sebagai file adegan berformat `.bscene`.
- **Acceptance Criteria:**
  - Pengguna dapat membuat scene baru di dalam chapter tertentu.
  - Setiap scene memiliki ID unik internal (UUID) dan nama tampilan.
  - File scene disimpan dalam subfolder chapter: `Scenes/Chapter_XX/<nama_scene>.bscene` (berbasis JSON).
  - Jika pengguna mengubah nama scene di UI, sistem memperbarui nama file dan referensi terkait secara aman.
  - Scene dapat dibuka dan dikelola melalui Scene Navigator di Left Sidebar.
- **Verification Method:** Test.

### REQ-FUNC-019 — Block-Based Scene Editor

- **ID:** REQ-FUNC-019
- **Title:** Editor scene berbasis block.
- **Statement:** Sistem harus menyediakan editor scene dengan susunan Action Block secara vertikal.
- **Acceptance Criteria:**
  - Setiap scene memiliki area penyusunan block.
  - Pengguna dapat menambahkan Action Block.
  - Pengguna dapat memilih block untuk mengubah propertinya.
  - Pengguna dapat menghapus block.
  - Pengguna dapat mengubah urutan block.
  - Pengguna dapat melihat susunan block secara visual.
  - Pengguna tidak perlu menulis kode untuk mengatur block.
- **Verification Method:** Test, Demonstration.

### REQ-FUNC-020 — Action Block Management

- **ID:** REQ-FUNC-020
- **Title:** Pengelolaan Action Block.
- **Statement:** Sistem harus menyediakan berbagai jenis Action Block untuk menyusun adegan visual novel.
- **Acceptance Criteria:**
  - Tersedia Background Block.
  - Tersedia Character Block.
  - Tersedia Dialogue Block.
  - Tersedia Narration Block.
  - Tersedia BGM Block.
  - Tersedia SFX Block.
  - Tersedia Transition Block.
  - Tersedia Wait Block.
  - Tersedia Choice Block (opsi pilihan cabang).
  - Tersedia Scene End Block (berpindah ke scene berikutnya, lompat ke scene tertentu, atau kembali ke Main Menu).
  - Setiap block memiliki konfigurasi sesuai fungsinya.
- **Verification Method:** Test.

### REQ-FUNC-021 — Background Configuration

- **ID:** REQ-FUNC-021
- **Title:** Pengaturan background scene.
- **Statement:** Sistem harus memungkinkan pengguna menentukan background yang digunakan dalam scene.
- **Acceptance Criteria:**
  - Pengguna dapat memilih background dari Asset Manager berdasarkan Asset ID.
  - Pengguna dapat mengganti background.
  - Pengguna dapat mengatur konfigurasi tampilan yang didukung.
  - Preview diperbarui setelah perubahan dilakukan.
- **Verification Method:** Test.

### REQ-FUNC-022 — Character Configuration and Sprite Management

- **ID:** REQ-FUNC-022
- **Title:** Pengaturan karakter, ekspresi sprite, dan posisi.
- **Statement:** Sistem harus memungkinkan pengguna menampilkan dan mengatur karakter di dalam scene menggunakan model Full Sprite per ekspresi dan preset posisi layar.
- **Acceptance Criteria:**
  - Pengguna dapat memilih karakter.
  - Pengguna dapat memilih ekspresi sprite (format satu gambar utuh/Full Sprite per pose/ekspresi, misalnya `senang`, `sedih`, `marah`).
  - Pengguna dapat menentukan posisi karakter di layar menggunakan preset ramah no-code: **`Left`**, **`Center`**, atau **`Right`** (dengan opsi penyesuaian offset jika dibutuhkan).
  - Pengguna dapat mengubah karakter atau ekspresi yang ditampilkan.
  - Pengguna dapat menyembunyikan atau mengeluarkan karakter dari scene.
  - Preview menampilkan konfigurasi karakter pada posisi kanvas yang tepat.
- **Verification Method:** Test.

### REQ-FUNC-023 — Dialogue Block Configuration and Formatting

- **ID:** REQ-FUNC-023
- **Title:** Pengaturan dialog scene dan formatting teks.
- **Statement:** Sistem harus memungkinkan pengguna memasukkan dialog dengan dukungan efek pengetikan (typewriter) dan pemformatan teks sederhana.
- **Acceptance Criteria:**
  - Pengguna dapat memilih dialog yang tersedia dari Story Manager atau mengetik langsung.
  - Sistem menampilkan nama pembicara dan isi teks dialog.
  - Runtime mendukung efek pengetikan (*typewriter effect*) dengan kecepatan teks yang dapat disesuaikan.
  - Sistem mendukung pemformatan teks inline sederhana bergaya Markdown (`**tebal**` dan `*miring*`).
  - Pengguna dapat mengatur konfigurasi tampilan dialog.
  - Dialog dapat diuji melalui preview.
- **Verification Method:** Test.

### REQ-FUNC-024 — Narration Block Configuration

- **ID:** REQ-FUNC-024
- **Title:** Pengaturan narasi scene.
- **Statement:** Sistem harus memungkinkan pengguna menambahkan elemen narasi ke dalam scene.
- **Acceptance Criteria:**
  - Pengguna dapat memilih narasi dari Story Manager atau mengetik langsung.
  - Sistem menampilkan teks narasi dengan dukungan formatting Markdown sederhana (`**tebal**`, `*miring*`).
  - Pengguna dapat mengatur tampilan narasi.
  - Narasi dapat ditampilkan melalui preview.
- **Verification Method:** Test.

### REQ-FUNC-025 — Audio Configuration and Seamless Playback

- **ID:** REQ-FUNC-025
- **Title:** Pengaturan audio scene dan pemutaran BGM berkesinambungan.
- **Statement:** Sistem harus memungkinkan pengguna mengatur BGM dan SFX dengan perilaku pemutaran musik latar yang berkesinambungan antar-scene (*seamless looping*).
- **Acceptance Criteria:**
  - Pengguna dapat memilih BGM dari Asset Manager berdasarkan Asset ID.
  - Pengguna dapat memilih SFX dari Asset Manager berdasarkan Asset ID.
  - Pengguna dapat mengatur volume dan status looping.
  - BGM yang sedang berputar di suatu scene otomatis terus berputar tanpa terputus (*seamless*) saat berpindah ke scene berikutnya, kecuali scene tujuan memiliki BGM Block baru atau instruksi Stop BGM / Fade Out.
  - Audio dapat diuji melalui preview.
- **Verification Method:** Test.

### REQ-FUNC-026 — Transition Configuration

- **ID:** REQ-FUNC-026
- **Title:** Pengaturan transisi.
- **Statement:** Sistem harus menyediakan konfigurasi transisi visual antaradegan atau antarperubahan tampilan.
- **Acceptance Criteria:**
  - Pengguna dapat memilih efek transisi yang tersedia.
  - Pengguna dapat mengatur durasi transisi.
  - Transisi dapat diuji melalui preview.
- **Verification Method:** Test.

### REQ-FUNC-027 — Scene Navigation

- **ID:** REQ-FUNC-027
- **Title:** Navigasi antar-scene.
- **Statement:** Sistem harus memungkinkan pengguna mengatur urutan dan hubungan antar-scene.
- **Acceptance Criteria:**
  - Pengguna dapat menentukan scene berikutnya.
  - Scene dapat dihubungkan dengan scene lain.
  - Sistem dapat mengidentifikasi scene yang tidak memiliki tujuan lanjutan ketika diperlukan.
  - Pengguna dapat melihat hubungan antar-scene.
- **Verification Method:** Test.

### REQ-FUNC-028 — Choice and Branching

- **ID:** REQ-FUNC-028
- **Title:** Pilihan dan percabangan cerita.
- **Statement:** Sistem harus menyediakan mekanisme pilihan pemain untuk menentukan kelanjutan cerita.
- **Acceptance Criteria:**
  - Pengguna dapat membuat beberapa pilihan.
  - Setiap pilihan dapat diarahkan ke scene tujuan.
  - Pemain dapat memilih opsi ketika game dimainkan.
  - Runtime menjalankan scene berdasarkan pilihan pemain.
  - Sistem memvalidasi tujuan setiap pilihan.
- **Verification Method:** Test, Demonstration.

### REQ-FUNC-029 — Scene End and System Navigation

- **ID:** REQ-FUNC-029
- **Title:** Mengakhiri scene dan navigasi sistem.
- **Statement:** Sistem harus menyediakan mekanisme untuk menentukan akhir suatu scene serta opsi kembali ke layar sistem (Main Menu).
- **Acceptance Criteria:**
  - Pengguna dapat menambahkan Scene End Block.
  - Scene End Block mendukung mode transisi: berpindah ke scene berikutnya, melompat ke scene tertentu, atau kembali ke Main Menu (*Return to Title*).
  - Sistem dapat mengidentifikasi akhir scene.
  - Sistem menjalankan perpindahan ke tujuan berikutnya jika telah ditentukan.
- **Verification Method:** Test.

### REQ-FUNC-029B — System Screens Configuration

- **ID:** REQ-FUNC-029B
- **Title:** Konfigurasi layar sistem bawaan (System Screens).
- **Statement:** Sistem harus menyediakan panel konfigurasi visual untuk layar sistem bawaan (Main Menu, Settings, Save/Load, In-Game Pause Menu) tanpa mengharuskan pengguna membuat logika UI secara manual.
- **Acceptance Criteria:**
  - Pengguna dapat mengonfigurasi Main Menu (Title Background, Title BGM, Game Logo/Title, dan Starting Scene).
  - Pengguna dapat mengonfigurasi tema visual antarmuka sistem (warna aksen, style dialogue box, font default).
  - Sistem menyediakan layar Settings bawaan (volume Master/BGM/SFX, kecepatan teks, mode fullscreen/windowed).
  - Sistem menyediakan In-Game Pause Menu bawaan yang dapat dipanggil saat gameplay berlangsung.
- **Verification Method:** Test, Demonstration.

## 3.2.6 Preview and Runtime

### REQ-FUNC-030 — Visual Novel Preview (Child Process Architecture)

- **ID:** REQ-FUNC-030
- **Title:** Preview visual novel dan arsitektur child process.
- **Statement:** Sistem harus menyediakan preview untuk melihat hasil scene maupun alur permainan lengkap yang diluncurkan sebagai proses mandiri terpisah (*child process*).
- **Acceptance Criteria:**
  - Editor meluncurkan runtime preview sebagai child process terpisah (`blanc_runtime`) dengan melewatkan path project dan argumen scene awal.
  - Kegagalan atau crash pada runtime tidak mempengaruhi stabilitas proses Editor utama.
  - Pengguna dapat menjalankan preview dari Main Menu (*Play from Beginning*) atau langsung dari scene yang dipilih (*Play from Scene*).
  - Background ditampilkan sesuai konfigurasi.
  - Karakter ditampilkan sesuai konfigurasi.
  - Dialog dan narasi ditampilkan.
  - BGM dan SFX dapat diputar.
  - Pilihan pemain dapat diuji.
  - In-Game Pause Menu dapat diakses selama preview.
  - Editor dapat menghentikan (*terminate*) proses preview kapan saja.
- **Verification Method:** Test, Demonstration.

### REQ-FUNC-031 — Runtime Execution and Player Controls

- **ID:** REQ-FUNC-031
- **Title:** Menjalankan visual novel dan kontrol pemain.
- **Statement:** Runtime harus dapat membaca dan menjalankan data visual novel yang dihasilkan oleh editor dengan kontrol pemain standar visual novel.
- **Acceptance Criteria:**
  - Runtime dapat memuat dan menampilkan Main Menu bawaan secara otomatis.
  - Runtime dapat memuat data scene cerita.
  - Runtime mendukung kontrol standar visual novel:
    - **Spasi / Klik Kiri / Enter:** Melanjutkan dialog ke baris berikutnya.
    - **Ctrl (Hold):** Mode Skip teks dialog.
    - **Tombol H:** Menyembunyikan antarmuka dialogue box (*Hide UI*).
    - **Tombol Esc:** Membuka In-Game Pause Menu.
    - **Mouse Scroll Up:** Membuka riwayat percakapan (*Backlog*).
  - Runtime dapat menampilkan background dan karakter pada posisi yang tepat.
  - Runtime dapat menjalankan dialog dengan typewriter effect dan memainkan audio.
  - Runtime dapat menjalankan branching pilihan pemain.
  - Runtime dapat menangani scene yang tidak valid dengan pesan kesalahan yang sesuai.
- **Verification Method:** Test, Demonstration.

### REQ-FUNC-032 — Runtime Save, Load, and Quick Save

- **ID:** REQ-FUNC-032
- **Title:** Penyimpanan, pemuatan, dan Quick Save/Load.
- **Statement:** Runtime harus menyediakan layar dan sistem Save & Load bawaan serta pintasan Quick Save dan Quick Load.
- **Acceptance Criteria:**
  - Pemain dapat menyimpan progres ke slot penyimpanan (dilengkapi thumbnail adegan, waktu simpan, dan nama scene).
  - Pemain dapat memuat progres yang tersimpan baik dari Main Menu maupun dari In-Game Menu.
  - Sistem mendukung **Quick Save (F5)** untuk menyimpan progres secara instan.
  - Sistem mendukung **Quick Load (F8 atau F9)** untuk memuat simpanan cepat secara instan.
  - Progres permainan dapat dipulihkan secara akurat sesuai data save.
- **Verification Method:** Test, Demonstration.

## 3.2.7 Validation and Export

### REQ-FUNC-033 — Project Validation

- **ID:** REQ-FUNC-033
- **Title:** Validasi project.
- **Statement:** Sistem harus memeriksa kelengkapan dan konsistensi project sebelum build.
- **Acceptance Criteria:**
  - Sistem memeriksa referensi aset.
  - Sistem memeriksa referensi dialog.
  - Sistem memeriksa konfigurasi scene.
  - Sistem memeriksa hubungan antar-scene.
  - Sistem mendeteksi aset yang hilang.
  - Sistem menampilkan daftar masalah dan lokasi masalah.
  - Sistem membedakan error yang menghalangi build dengan warning.
- **Verification Method:** Test.

### REQ-FUNC-034 — Game Build and Standalone Export

- **ID:** REQ-FUNC-034
- **Title:** Build dan export game mandiri.
- **Statement:** Sistem harus memungkinkan pengguna menghasilkan visual novel menjadi satu folder aplikasi mandiri yang siap dimainkan tanpa memerlukan editor.
- **Acceptance Criteria:**
  - Pengguna dapat memulai proses build.
  - Sistem melakukan validasi project sebelum build.
  - Sistem menghasilkan output dalam satu folder mandiri berisi file executable **`<NamaGame>.exe`** (untuk Windows) beserta bundel data/aset permainan.
  - Game hasil export dapat langsung dijalankan oleh pemain tanpa perlu menginstal Blanc Engine.
  - Sistem menampilkan progress bar dan status penyelesaian proses build.
  - Sistem memberikan log dan informasi jika build mengalami kegagalan.
- **Verification Method:** Test, Demonstration.

### REQ-FUNC-035 — Project Backup

- **ID:** REQ-FUNC-035
- **Title:** Backup project.
- **Statement:** Sistem sebaiknya menyediakan mekanisme backup untuk melindungi data project.
- **Acceptance Criteria:**
  - Pengguna dapat membuat backup.
  - Backup mencakup data project dan aset yang dibutuhkan.
  - Pengguna dapat menentukan lokasi backup.
  - Backup dapat digunakan untuk memulihkan project.
- **Verification Method:** Test.

---

# 3.3 Quality of Service

## 3.3.1 Performance

### REQ-PERF-001 — Application Responsiveness

- **ID:** REQ-PERF-001
- **Title:** Responsivitas aplikasi.
- **Statement:** Antarmuka aplikasi harus tetap responsif selama operasi normal.
- **Rationale:** Pengguna harus dapat berinteraksi dengan editor tanpa gangguan yang menghambat pekerjaan.
- **Acceptance Criteria:**
  - Interaksi UI umum memiliki target respons maksimal 200 ms pada perangkat pengujian yang memenuhi spesifikasi minimum.
  - Operasi berat tidak boleh menyebabkan UI mengalami freeze berkepanjangan.
  - Proses import dan build harus menampilkan status proses.
- **Verification Method:** Test, Analysis.

### REQ-PERF-002 — Resource Efficiency

- **ID:** REQ-PERF-002
- **Title:** Efisiensi penggunaan sumber daya.
- **Statement:** Sistem harus mengelola penggunaan CPU, RAM, dan storage secara efisien.
- **Acceptance Criteria:**
  - Aset tidak digunakan tidak harus dimuat seluruhnya ke memori.
  - Sistem mendukung pemuatan aset sesuai kebutuhan.
  - Preview thumbnail tidak menggunakan resolusi penuh apabila tidak diperlukan.
  - Proses import dan rendering memperhatikan penggunaan memori.
- **Verification Method:** Analysis, Test.

### REQ-PERF-003 — Project Loading

- **ID:** REQ-PERF-003
- **Title:** Waktu pembukaan project.
- **Statement:** Sistem harus memuat project dengan waktu yang wajar berdasarkan ukuran project dan jumlah aset.
- **Acceptance Criteria:**
  - Sistem memberikan indikator loading.
  - Sistem tidak memblokir seluruh antarmuka selama pemuatan project.
  - Sistem memberikan pesan jika ditemukan file yang tidak dapat dibaca.
- **Verification Method:** Test.

Target numerik untuk ukuran project dan waktu loading akan ditentukan setelah benchmark prototipe.

## 3.3.2 Security

### REQ-SEC-001 — Local Project Security

- **ID:** REQ-SEC-001
- **Title:** Keamanan data project.
- **Statement:** Sistem harus menjaga integritas file project ketika melakukan operasi penyimpanan dan perubahan data.
- **Acceptance Criteria:**
  - Sistem tidak mengubah file di luar direktori project tanpa persetujuan pengguna.
  - Operasi penghapusan memerlukan konfirmasi.
  - Kegagalan penyimpanan tidak boleh menyebabkan sistem menganggap data berhasil disimpan.
- **Verification Method:** Test, Inspection.

### REQ-SEC-002 — External Asset Validation

- **ID:** REQ-SEC-002
- **Title:** Validasi file eksternal.
- **Statement:** Sistem harus memvalidasi file eksternal sebelum memasukkannya ke project.
- **Acceptance Criteria:**
  - Sistem memeriksa tipe file.
  - Sistem menangani file yang rusak atau tidak valid.
  - Sistem tidak mengeksekusi file aset sebagai program.
  - Sistem membatasi pemrosesan file yang tidak didukung.
- **Verification Method:** Test.

### REQ-SEC-003 — Project Path Safety

- **ID:** REQ-SEC-003
- **Title:** Validasi lokasi project.
- **Statement:** Sistem harus memvalidasi operasi filesystem agar tidak mengakses atau mengubah lokasi di luar cakupan yang diizinkan.
- **Acceptance Criteria:**
  - Sistem memvalidasi path project.
  - Sistem menangani symbolic link dan path yang tidak valid secara aman.
  - Sistem memberikan pesan jika tidak memiliki izin akses.
- **Verification Method:** Test, Analysis.

## 3.3.3 Reliability

### REQ-REL-001 — Data Integrity

- **ID:** REQ-REL-001
- **Title:** Integritas data project.
- **Statement:** Sistem harus mempertahankan konsistensi data project ketika melakukan operasi penyimpanan.
- **Acceptance Criteria:**
  - Data yang disimpan dapat dibaca kembali.
  - Sistem memvalidasi data project.
  - Kegagalan operasi tidak boleh meninggalkan data dalam kondisi yang dianggap valid secara keliru.
- **Verification Method:** Test.

### REQ-REL-002 — Error Handling

- **ID:** REQ-REL-002
- **Title:** Penanganan kesalahan.
- **Statement:** Sistem harus memberikan informasi yang dapat dipahami ketika terjadi kesalahan.
- **Acceptance Criteria:**
  - Pesan error menjelaskan masalah yang terjadi.
  - Sistem tidak menutup aplikasi secara paksa untuk kesalahan yang dapat ditangani.
  - Pengguna mendapatkan informasi tindakan yang dapat dilakukan.
- **Verification Method:** Test.

### REQ-REL-003 — Missing Asset Handling

- **ID:** REQ-REL-003
- **Title:** Penanganan aset hilang.
- **Statement:** Sistem harus dapat mendeteksi dan menangani aset yang tidak ditemukan.
- **Acceptance Criteria:**
  - Sistem mengidentifikasi referensi aset yang hilang.
  - Sistem menampilkan lokasi penggunaan aset.
  - Sistem menyediakan mekanisme untuk memperbaiki referensi aset.
- **Verification Method:** Test.

## 3.3.4 Availability

### REQ-AVAIL-001 — Offline Operation

- **ID:** REQ-AVAIL-001
- **Title:** Operasional tanpa internet.
- **Statement:** Fitur inti Blanc Engine harus dapat digunakan tanpa koneksi internet.
- **Acceptance Criteria:**
  - Pengguna dapat membuat project secara offline.
  - Pengguna dapat mengelola aset secara offline.
  - Pengguna dapat mengedit story dan scene secara offline.
  - Pengguna dapat melakukan preview secara offline.
  - Proses build lokal tidak mewajibkan koneksi internet setelah seluruh dependency yang diperlukan tersedia.
- **Verification Method:** Test.

## 3.3.5 Observability

### REQ-OBS-001 — Application Logging

- **ID:** REQ-OBS-001
- **Title:** Logging aplikasi.
- **Statement:** Sistem harus menyediakan mekanisme pencatatan informasi operasional dan kesalahan aplikasi.
- **Acceptance Criteria:**
  - Sistem mencatat error penting.
  - Sistem mencatat kegagalan operasi project.
  - Log dapat digunakan untuk membantu proses debugging.
  - Log tidak boleh secara tidak sengaja membocorkan data sensitif pengguna.
- **Verification Method:** Inspection, Test.

---

# 3.4 Compliance

## 3.4.1 Open-Source Compliance

### REQ-COMP-001 — Open-Source Distribution and Copyleft License

- **ID:** REQ-COMP-001
- **Title:** Distribusi kode sumber secara terbuka dan lisensi copyleft GPLv3.
- **Statement:** Kode sumber Blanc Engine harus tersedia melalui repositori publik di bawah lisensi **GNU General Public License v3 (GPLv3)**.
- **Acceptance Criteria:**
  - Source code Blanc Engine dapat diakses secara publik dengan file LICENSE (GPLv3).
  - Setiap pihak yang memodifikasi, memperluas, membuat turunan, atau me-rebranding Blanc Engine diwajibkan secara hukum untuk merilis kembali seluruh source code modifikasinya di bawah lisensi GPLv3.
  - Proyek visual novel, naskah cerita, ilustrasi, musik, dan aset yang dibuat oleh kreator menggunakan Blanc Engine merupakan hak cipta eksklusif kreator dan bebas didistribusikan atau dijual secara komersial tanpa terikat ketentuan copyleft GPL.
  - Repository menyediakan panduan kontribusi dan dokumentasi lisensi pihak ketiga.
- **Verification Method:** Inspection.

## 3.4.2 Third-Party Asset Compliance

### REQ-COMP-002 — Kepatuhan penggunaan aset.
- **ID:** REQ-COMP-002
- **Title:** Kepatuhan lisensi aset.
- **Statement:** Sistem harus memungkinkan pengelolaan aset tanpa mengabaikan hak penggunaan dan lisensi yang berlaku.
- **Acceptance Criteria:**
  - Pengguna bertanggung jawab atas hak penggunaan aset yang diimpor.
  - Sistem tidak mengklaim kepemilikan aset milik pengguna.
  - Dokumentasi menjelaskan tanggung jawab penggunaan aset pihak ketiga.
- **Verification Method:** Inspection.

---

# 3.5 Design and Implementation

## 3.5.1 Installation

### REQ-INST-001 — Desktop Installation

- **ID:** REQ-INST-001
- **Title:** Instalasi aplikasi.
- **Statement:** Blanc Engine harus dapat didistribusikan sebagai aplikasi desktop yang dapat dipasang atau dijalankan pada sistem operasi yang didukung.
- **Acceptance Criteria:**
  - Paket distribusi tersedia untuk platform yang ditetapkan.
  - Pengguna dapat menjalankan aplikasi tanpa harus membangun source code sendiri.
  - Prasyarat instalasi dijelaskan dalam dokumentasi.
- **Verification Method:** Demonstration, Test.

Target sistem operasi akan ditetapkan dalam keputusan platform.

## 3.5.2 Build and Delivery

### REQ-BUILD-001 — Reproducible Application Build

- **ID:** REQ-BUILD-001
- **Title:** Konsistensi proses build.
- **Statement:** Source code Blanc Engine harus dapat dibangun menggunakan toolchain dan dependency yang terdokumentasi.
- **Acceptance Criteria:**
  - Versi dependency dapat dilacak.
  - Proses build terdokumentasi.
  - Kesalahan build dapat diidentifikasi.
  - Kontributor dapat mengikuti instruksi build.
- **Verification Method:** Test, Inspection.

### REQ-BUILD-002 — Release Packaging

- **ID:** REQ-BUILD-002
- **Title:** Pengemasan aplikasi.
- **Statement:** Sistem build harus dapat menghasilkan paket distribusi Blanc Engine.
- **Acceptance Criteria:**
  - Paket distribusi memiliki versi.
  - Paket dapat diverifikasi.
  - Dokumentasi instalasi tersedia.
- **Verification Method:** Test.

## 3.5.3 Distribution

### REQ-DIST-001 — Project Portability

- **ID:** REQ-DIST-001
- **Title:** Portabilitas project.
- **Statement:** Project Blanc Engine harus dapat dipindahkan ke lokasi lain tanpa kehilangan struktur data dan referensi aset internal.
- **Acceptance Criteria:**
  - Project dapat disalin ke folder lain.
  - Referensi aset menggunakan mekanisme yang tidak bergantung pada absolute path komputer asal.
  - Project dapat dibuka kembali dari lokasi baru.
  - Sistem mendeteksi file yang hilang setelah pemindahan.
- **Verification Method:** Test.

## 3.5.4 Maintainability

### REQ-MAINT-001 — Modular Architecture

- **ID:** REQ-MAINT-001
- **Title:** Arsitektur modular.
- **Statement:** Source code Blanc Engine harus disusun secara modular berdasarkan tanggung jawab sistem.
- **Acceptance Criteria:**
  - Modul project management dipisahkan dari asset management.
  - Story management dipisahkan dari scene management.
  - Editor dipisahkan secara tanggung jawab dari runtime.
  - Sistem rendering dan audio memiliki tanggung jawab yang jelas.
  - Modul memiliki batas dependensi yang terdokumentasi.
- **Verification Method:** Inspection.

### REQ-MAINT-002 — Documentation

- **ID:** REQ-MAINT-002
- **Title:** Dokumentasi pengembangan.
- **Statement:** Repository harus menyediakan dokumentasi yang membantu pengembangan dan kontribusi.
- **Acceptance Criteria:**
  - Tersedia README.
  - Tersedia dokumentasi instalasi.
  - Tersedia dokumentasi build.
  - Tersedia dokumentasi arsitektur.
  - Tersedia panduan kontribusi.
- **Verification Method:** Inspection.

### REQ-MAINT-003 — Testing Support

- **ID:** REQ-MAINT-003
- **Title:** Dukungan automated testing.
- **Statement:** Komponen utama Blanc Engine harus dapat diuji secara otomatis.
- **Acceptance Criteria:**
  - Modul project memiliki pengujian.
  - Modul story memiliki pengujian.
  - Modul scene memiliki pengujian.
  - Modul runtime memiliki pengujian.
  - Pengujian dapat dijalankan melalui toolchain Rust.
- **Verification Method:** Test, Inspection.

## 3.5.5 Reusability

### REQ-REUSE-001 — Reusable Runtime

- **ID:** REQ-REUSE-001
- **Title:** Runtime yang dapat digunakan kembali.
- **Statement:** Runtime harus dirancang agar dapat menjalankan berbagai project visual novel yang dibuat menggunakan Blanc Engine.
- **Acceptance Criteria:**
  - Runtime tidak bergantung pada identitas satu project tertentu.
  - Runtime dapat membaca struktur data project yang didukung.
  - Perubahan project tidak mengharuskan runtime dibuat ulang secara manual.
- **Verification Method:** Test, Inspection.

## 3.5.6 Portability

### REQ-PORT-001 — Cross-Platform Architecture

- **ID:** REQ-PORT-001
- **Title:** Arsitektur lintas platform.
- **Statement:** Arsitektur Blanc Engine SHOULD mempertimbangkan dukungan beberapa sistem operasi desktop.
- **Acceptance Criteria:**
  - Modul yang berinteraksi dengan sistem operasi dipisahkan dari logika utama.
  - Penggunaan filesystem dan windowing memperhatikan perbedaan platform.
  - Dependency yang digunakan diperiksa kompatibilitasnya.
- **Verification Method:** Inspection, Analysis.

Platform yang didukung secara resmi masih perlu ditentukan.

## 3.5.7 Cost

### REQ-COST-001 — Development Cost Efficiency

- **ID:** REQ-COST-001
- **Title:** Efisiensi biaya pengembangan.
- **Statement:** Pengembangan Blanc Engine harus mengutamakan penggunaan teknologi open-source dan menghindari ketergantungan berbayar yang tidak diperlukan.
- **Acceptance Criteria:**
  - Dependency utama dapat digunakan tanpa biaya lisensi komersial wajib.
  - Dependency pihak ketiga terdokumentasi.
  - Penggunaan layanan berbayar tidak diwajibkan untuk fitur inti.
- **Verification Method:** Inspection.

## 3.5.8 Deadline

### REQ-DEAD-001 — Incremental Development

- **ID:** REQ-DEAD-001
- **Title:** Pengembangan bertahap.
- **Statement:** Pengembangan Blanc Engine harus dilakukan melalui tahapan yang memiliki ruang lingkup dan kriteria penyelesaian yang jelas.
- **Acceptance Criteria:**
  - MVP memiliki daftar fitur yang terdefinisi.
  - Setiap tahap memiliki target pengujian.
  - Fitur lanjutan tidak menghambat penyelesaian fitur inti.
- **Verification Method:** Inspection.

## 3.5.9 Proof of Concept

### REQ-POC-001 — Core Workflow Validation

- **ID:** REQ-POC-001
- **Title:** Validasi alur pembuatan visual novel.
- **Statement:** Sebelum pengembangan penuh, sistem harus memiliki prototipe yang membuktikan kelayakan workflow utama Blanc Engine.
- **Acceptance Criteria:**
  - Pengguna dapat membuat project.
  - Pengguna dapat mengimpor aset.
  - Pengguna dapat memasukkan cerita.
  - Pengguna dapat menyusun scene menggunakan block.
  - Pengguna dapat melakukan preview.
  - Prototipe dapat menjalankan satu visual novel sederhana.
- **Verification Method:** Demonstration, Test.

## 3.5.10 Change Management

### REQ-CM-001 — Project Format Versioning

- **ID:** REQ-CM-001
- **Title:** Versioning format project.
- **Statement:** Format penyimpanan project harus memiliki informasi versi untuk mendukung pengembangan dan kompatibilitas.
- **Acceptance Criteria:**
  - Setiap project menyimpan versi format.
  - Sistem dapat mengidentifikasi versi project.
  - Sistem memberikan informasi jika project menggunakan format yang tidak didukung.
  - Perubahan format yang tidak kompatibel harus didokumentasikan.
- **Verification Method:** Test, Inspection.

### REQ-CM-002 — Migration Support

- **ID:** REQ-CM-002
- **Title:** Migrasi project.
- **Statement:** Sistem sebaiknya menyediakan mekanisme migrasi ketika format project mengalami perubahan.
- **Acceptance Criteria:**
  - Sistem dapat mendeteksi kebutuhan migrasi.
  - Pengguna mendapatkan informasi sebelum migrasi.
  - Data project tidak boleh diubah tanpa mekanisme perlindungan.
  - Sistem menyediakan mekanisme pemulihan jika migrasi gagal.
- **Verification Method:** Test.

---

# 3.6 AI/ML

## 3.6.1 Model Specification

Tidak berlaku untuk ruang lingkup awal Blanc Engine.

Blanc Engine tidak dirancang untuk menggunakan machine learning sebagai komponen inti sistem.

## 3.6.2 Data Management

Tidak berlaku untuk ruang lingkup awal karena sistem tidak menggunakan dataset pelatihan atau model machine learning.

## 3.6.3 Guardrails

Tidak berlaku untuk ruang lingkup awal.

## 3.6.4 Ethics

Tidak berlaku untuk ruang lingkup awal.

## 3.6.5 Human-in-the-Loop

Tidak berlaku untuk ruang lingkup awal.

## 3.6.6 Model Lifecycle and Operations

Tidak berlaku untuk ruang lingkup awal.

---

# 4. Verification

Verifikasi dilakukan untuk memastikan setiap kebutuhan yang ditentukan dalam dokumen ini telah diimplementasikan sesuai acceptance criteria.

Metode verifikasi yang digunakan:

| Metode | Penjelasan |
|---|---|
| Test | Menguji fungsi melalui input dan output sistem. |
| Analysis | Menganalisis performa, penggunaan sumber daya, atau karakteristik sistem. |
| Inspection | Memeriksa source code, konfigurasi, dokumentasi, dan struktur project. |
| Demonstration | Memperlihatkan fitur berjalan sesuai kebutuhan. |

## 4.1 Verification Matrix

| Requirement ID | Verification Method | Status |
|---|---|---|
| REQ-FUNC-001 – 005 | Test | Not Started |
| REQ-FUNC-006 – 007 | Test, Demonstration | Not Started |
| REQ-FUNC-008 – 012 | Test | Not Started |
| REQ-FUNC-013 – 017 | Test | Not Started |
| REQ-FUNC-018 – 029 | Test, Demonstration | Not Started |
| REQ-FUNC-030 – 032 | Test, Demonstration | Not Started |
| REQ-FUNC-033 – 035 | Test, Demonstration | Not Started |
| REQ-PERF-001 – 003 | Test, Analysis | Not Started |
| REQ-SEC-001 – 003 | Test, Inspection | Not Started |
| REQ-REL-001 – 003 | Test | Not Started |
| REQ-AVAIL-001 | Test | Not Started |
| REQ-OBS-001 | Inspection, Test | Not Started |
| REQ-COMP-001 – 002 | Inspection | Not Started |
| REQ-INST-001 | Test, Demonstration | Not Started |
| REQ-BUILD-001 – 002 | Test, Inspection | Not Started |
| REQ-DIST-001 | Test | Not Started |
| REQ-MAINT-001 – 003 | Inspection, Test | Not Started |
| REQ-REUSE-001 | Test, Inspection | Not Started |
| REQ-PORT-001 | Inspection, Analysis | Not Started |
| REQ-COST-001 | Inspection | Not Started |
| REQ-DEAD-001 | Inspection | Not Started |
| REQ-POC-001 | Test, Demonstration | Not Started |
| REQ-CM-001 – 002 | Test, Inspection | Not Started |

## 4.2 MVP Acceptance

MVP Blanc Engine dinyatakan memenuhi persyaratan awal apabila:

1. Pengguna dapat membuat project baru.
2. Pengguna dapat menentukan lokasi project.
3. Pengguna dapat mengimpor dan mengelola aset.
4. Pengguna dapat memasukkan dan mengelola cerita.
5. Pengguna dapat membuat chapter dan scene.
6. Pengguna dapat menyusun scene menggunakan Action Block.
7. Pengguna dapat mengatur background, karakter, dialog, narasi, BGM, dan SFX.
8. Pengguna dapat melakukan preview visual novel.
9. Runtime dapat menjalankan visual novel yang dibuat.
10. Pengguna dapat menghasilkan game standalone.
11. Project dapat disimpan dan dibuka kembali.
12. Seluruh proses pembuatan visual novel dapat dilakukan tanpa menulis kode.

---

# 5. Appendixes

## 5.1 Proposed Project Structure

Struktur berikut merupakan rancangan awal untuk penyimpanan project Blanc Engine.

```text
MyVisualNovel/
│
├── MyVisualNovel.blanc          <-- Manifest project (format JSON)
│
├── Assets/
│   ├── Backgrounds/
│   ├── Characters/
│   │   ├── Alice/
│   │   │   ├── happy.png        <-- Full Sprite per ekspresi
│   │   │   └── sad.png
│   │   └── Bob/
│   ├── BGM/
│   ├── SFX/
│   ├── Fonts/
│   └── UI/                      <-- Aset khusus Main Menu, Logo, UI System
│
├── Story/
│   └── Chapters/
│       ├── chapter_01.bstory    <-- Data naskah internal
│       └── chapter_02.bstory
│
├── Scenes/
│   ├── Chapter_01/
│   │   ├── 01_gerbang_sekolah.bscene  <-- Action blocks adegan
│   │   └── 02_ruang_kelas.bscene
│   └── Chapter_02/
│       └── 01_halaman_belakang.bscene
│
├── Settings/
│   └── system_screens.bconfig   <-- Konfigurasi Main Menu, UI Theme, dll.
│
├── Builds/
│
└── Backups/
```

Struktur ini menggunakan file manifest berformat `<NamaProject>.blanc` yang berbasis JSON, dan memisahkan konfigurasi layar sistem (System Screens) dengan data adegan cerita (Scenes).

## 5.2 Proposed Scene Block Types

| Block | Fungsi |
|---|---|
| Background | Mengatur background scene. |
| Character | Menampilkan karakter. |
| Dialogue | Menampilkan dialog. |
| Narration | Menampilkan narasi. |
| BGM | Mengatur musik latar. |
| SFX | Memutar efek suara. |
| Transition | Mengatur transisi visual. |
| Wait | Memberikan jeda dalam urutan aksi. |
| Choice | Menampilkan pilihan kepada pemain. |
| Scene End | Mengakhiri scene, melompat ke scene lain, atau kembali ke Main Menu (*Return to Title*). |

## 5.3 Proposed Main Application Workflow

```text
Launch Blanc Engine
        |
        v
    Welcome Page
        |
        v
   Create Project
        |
        v
  Select Project Folder
        |
        v
   Create Project
        |
        v
    Asset Manager
        |
        v
    Story Manager
        |
        v
     Scene Editor
        |
        v
    Preview Player
        |
        v
   Project Validation
        |
        v
     Build & Export
        |
        v
  Standalone Visual Novel
```

Pengguna dapat kembali ke halaman sebelumnya kapan saja. Akses menuju halaman selanjutnya bergantung pada validasi tahapan sebelumnya.

## 5.4 Proposed Development Phases

### Phase 1 — Foundation

- Project initialization.
- Project structure.
- Main application layout.
- Workflow navigation.
- Asset Manager.
- Story Manager.

### Phase 2 — Scene Editor

- Scene management.
- Block-based editor.
- Action Block system.
- Background configuration.
- Character configuration.
- Dialogue dan narration.
- Audio configuration.

### Phase 3 — Runtime

- Visual renderer.
- Dialogue runtime.
- Character rendering.
- Audio playback.
- Scene navigation.
- Choice dan branching.

### Phase 4 — Preview and Export

- Preview Player.
- Project validation.
- Runtime integration.
- Build system.
- Standalone game export.

### Phase 5 — Advanced Features

- Advanced branching.
- Multiple endings.
- Save/load system.
- Advanced character expressions.
- Additional transitions.
- UI customization.
- Plugin system.

---

**End of Document**

**Blanc Engine — Turn Your Stories Into Games.**