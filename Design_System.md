# BLANC ENGINE
# DESIGN SYSTEM
**Document Version:** 0.1  
**Status:** Initial Draft  
**Product:** Blanc Engine  
**Technology:** Rust + Iced  
**Document Type:** UI/UX Design Specification  
**Last Updated:** 12 November 2026

---

# 1. DESIGN SYSTEM OVERVIEW

## 1.1 Purpose

Dokumen Design System ini menjadi pedoman standar desain antarmuka dan pengalaman pengguna (UI/UX) untuk Blanc Engine.

Dokumen ini mendefinisikan identitas visual, sistem warna, tipografi, layout, komponen antarmuka, interaksi, serta pedoman desain setiap halaman aplikasi.

Tujuannya adalah memastikan seluruh antarmuka Blanc Engine memiliki konsistensi visual, mudah digunakan, profesional, dan mendukung proses pembuatan visual novel tanpa memerlukan kemampuan pemrograman.

## 1.2 Scope

Design System ini mencakup:

- Identitas visual Blanc Engine.
- Warna dan tema aplikasi.
- Tipografi.
- Layout dan sistem spacing.
- Iconography dan penggunaan aset visual.
- Komponen UI.
- Pedoman desain halaman.
- Interaction design.
- Feedback dan state management.
- Accessibility.
- Responsive desktop layout.
- Pedoman implementasi desain menggunakan Iced.
- Konsistensi desain dan pengembangan komponen.

## 1.3 Design Goals

Blanc Engine memiliki tujuan desain sebagai berikut:

1. Menghasilkan antarmuka desktop yang profesional dan modern.
2. Mempermudah pengguna membuat visual novel tanpa menulis kode.
3. Menampilkan informasi secara padat tetapi tetap mudah dibaca.
4. Meminimalkan kompleksitas penggunaan melalui guided workflow.
5. Menyediakan pengalaman editing yang konsisten.
6. Mendukung pengembangan fitur dalam jangka panjang.
7. Memisahkan pengalaman editing dan gameplay runtime.

## 1.4 Design Philosophy

Blanc Engine mengadopsi filosofi:

**Professional, Functional, Minimal, and Creator-Focused.**

Antarmuka harus terasa seperti perangkat lunak pengembangan game profesional, tetapi tidak boleh menjadi terlalu kompleks bagi pengguna yang tidak memiliki latar belakang teknis.

Desain mengutamakan fungsi dibanding dekorasi. Setiap elemen UI harus memiliki tujuan yang jelas.

## 1.5 Design References

Referensi pendekatan desain:

- Unreal Engine: struktur workspace dan panel editor.
- Godot Engine: kesederhanaan workflow dan pengelolaan project.
- Unity Editor: konsep Inspector dan pengaturan objek.
- Modern Desktop Applications: konsistensi komponen, tipografi, dan navigasi.

Blanc Engine tidak bertujuan meniru antarmuka produk tersebut secara langsung. Referensi hanya digunakan sebagai inspirasi pendekatan desain.

---

# 2. BRAND AND VISUAL DIRECTION

## 2.1 Brand Identity

Blanc Engine merupakan desktop application untuk membuat visual novel melalui pendekatan no-code.

Identitas visual harus mencerminkan:

- Profesional.
- Modern.
- Minimalis.
- Presisi.
- Kreatif tetapi tidak berlebihan.
- Mudah digunakan.

## 2.2 Visual Direction

Arah visual utama:

**Dark Neutral + Indigo Accent**

Karakter desain menggunakan warna netral gelap dengan aksen indigo yang digunakan secara terbatas.

Karakteristik visual:

- Matte dark surfaces.
- Subtle borders.
- Minimal shadows.
- Compact interface.
- Clear visual hierarchy.
- Consistent component styling.
- Minimal decorative elements.

## 2.3 Design Character

| Attribute | Specification |
|---|---|
| Design Style | Professional Desktop Editor |
| Visual Tone | Modern and Minimal |
| Density | Compact |
| Primary Theme | Dark |
| Secondary Theme | Light |
| Accent | Indigo |
| Corner Style | Subtle Rounded |
| Elevation | Minimal |
| Decoration | Minimal |

## 2.4 Logo and Branding

Logo Blanc Engine harus mengikuti karakter visual aplikasi.

Pedoman:

- Mengutamakan bentuk sederhana dan mudah dikenali.
- Tidak menggunakan terlalu banyak detail.
- Dapat digunakan pada dark dan light backgrounds.
- Tetap terlihat jelas dalam ukuran kecil.
- Tidak menggunakan efek visual berlebihan.

Detail final logo dan brand assets: **TBD**.

---

# 3. COLOR SYSTEM

## 3.1 Color Philosophy

Sistem warna Blanc Engine menggunakan neutral dark palette sebagai fondasi.

Indigo berfungsi sebagai primary accent untuk menandai interaksi, elemen aktif, dan tindakan utama.

Warna semantic digunakan hanya untuk menyampaikan status atau informasi tertentu.

## 3.2 Dark Theme - Neutral Colors

| Token | Hex | Usage |
|---|---|---|
| `background-primary` | `#17191D` | Background utama aplikasi |
| `background-secondary` | `#1E2126` | Sidebar dan panel utama |
| `surface-primary` | `#24282E` | Card dan komponen |
| `surface-elevated` | `#2B3037` | Dropdown, modal, elevated surface |
| `surface-hover` | `#303640` | Hover state |
| `border-default` | `#353B44` | Border standar |
| `border-subtle` | `#2B3037` | Pemisah visual ringan |

## 3.3 Text Colors

| Token | Hex | Usage |
|---|---|---|
| `text-primary` | `#E6E9ED` | Teks utama |
| `text-secondary` | `#A0A7B2` | Deskripsi dan informasi sekunder |
| `text-muted` | `#737C89` | Metadata dan placeholder |
| `text-disabled` | `#555D68` | Teks nonaktif |
| `text-on-accent` | `#FFFFFF` | Teks di atas primary accent |

## 3.4 Accent Colors

| Token | Hex | Usage |
|---|---|---|
| `accent-primary` | `#7C8CFF` | Primary actions |
| `accent-hover` | `#8D9AFF` | Hover |
| `accent-pressed` | `#6879EB` | Pressed |
| `accent-subtle` | `#292D3A` | Selected background |
| `accent-border` | `#7C8CFF` | Selected border |

## 3.5 Semantic Colors

| Token | Hex | Usage |
|---|---|---|
| `status-success` | `#55B88A` | Berhasil dan valid |
| `status-warning` | `#E6AD57` | Peringatan |
| `status-error` | `#E16F78` | Error dan kegagalan |
| `status-info` | `#69A9E8` | Informasi |

Semantic colors tidak boleh digunakan sebagai dekorasi.

## 3.6 Color Usage Rules

1. Background utama menggunakan `background-primary`.
2. Panel navigasi menggunakan `background-secondary`.
3. Komponen menggunakan `surface-primary`.
4. Primary action menggunakan `accent-primary`.
5. Elemen terpilih menggunakan `accent-subtle` dan `accent-border`.
6. Error harus menggunakan `status-error`.
7. Warning harus menggunakan `status-warning`.
8. Jangan menggunakan indigo pada seluruh komponen hanya untuk memperindah tampilan.
9. Hindari penggunaan gradient sebagai dekorasi standar.
10. Pastikan teks memiliki kontras yang memadai.

## 3.7 Light Theme

Blanc Engine menyediakan light theme sebagai alternatif.

Light theme harus mempertahankan struktur visual dan identitas warna yang sama dengan dark theme.

| Token | Proposed Hex |
|---|---|
| `background-primary` | `#F5F6F8` |
| `background-secondary` | `#ECEEF2` |
| `surface-primary` | `#FFFFFF` |
| `surface-elevated` | `#FFFFFF` |
| `surface-hover` | `#F0F1F5` |
| `border-default` | `#D9DDE5` |
| `text-primary` | `#20242B` |
| `text-secondary` | `#626B78` |
| `text-muted` | `#89919D` |
| `accent-primary` | `#6678ED` |

Nilai light theme merupakan rancangan awal dan perlu diuji untuk kontras serta keterbacaan sebelum implementasi final.

## 3.8 Theme Behavior

Pilihan tema:

- Dark.
- Light.
- System Default.

Pengguna dapat mengubah tema melalui Settings.

Preferensi tema disimpan pada pengaturan aplikasi dan dipertahankan ketika aplikasi dibuka kembali.

---

# 4. TYPOGRAPHY SYSTEM

## 4.1 Typography Philosophy

Tipografi harus mendukung penggunaan aplikasi desktop dalam sesi pengembangan yang panjang.

Prioritas:

- Readability.
- Consistency.
- Clear hierarchy.
- Compact presentation.
- Comfortable reading.

## 4.2 Primary Font

**Inter** menjadi font utama untuk seluruh antarmuka.

Penggunaan:

| Style | Usage |
|---|---|
| Inter Regular | Body text |
| Inter Medium | Labels and controls |
| Inter SemiBold | Headings |
| Inter Bold | Important emphasis |

## 4.3 Monospace Font

**JetBrains Mono** digunakan untuk informasi teknis.

Contoh penggunaan:

- Log output.
- File path.
- Error details.
- Technical identifiers.
- Debug information.

## 4.4 Typography Scale

| Token | Size | Weight | Usage |
|---|---:|---|---|
| `heading-xl` | 22 px | Semibold | Halaman utama |
| `heading-lg` | 18 px | Semibold | Section heading |
| `heading-md` | 16 px | Semibold | Panel heading |
| `heading-sm` | 14 px | Semibold | Component heading |
| `body-md` | 14 px | Regular | Body utama |
| `body-sm` | 13 px | Regular | Teks komponen |
| `label-md` | 12 px | Medium | Label |
| `label-sm` | 11 px | Medium | Metadata |
| `code-md` | 13 px | Regular | Informasi teknis |

## 4.5 Typography Rules

- Gunakan hierarchy secara konsisten.
- Hindari terlalu banyak variasi font weight.
- Jangan menggunakan ukuran terlalu kecil untuk informasi penting.
- Gunakan monospace hanya untuk informasi teknis.
- Hindari penggunaan uppercase pada seluruh teks.
- Gunakan uppercase secara terbatas untuk kategori atau label block.

## 4.6 Interface Language & Localization

- Antarmuka Blanc Engine Editor pada MVP menggunakan **Bahasa Inggris standar industri game development** (misal: *File, Project, Asset Manager, Scene Editor, Inspector, Play, Settings*).
- Semua string antarmuka dikelola secara terpusat melalui modul teks / lokalisasi agar siap mendukung multi-bahasa (termasuk Bahasa Indonesia) pada rilis mendatang.

---

# 5. LAYOUT AND SPACING SYSTEM

## 5.1 Layout Philosophy

Blanc Engine menggunakan multi-panel desktop workspace.

Workspace harus memaksimalkan area editing dan memudahkan pengguna berpindah antara project explorer, scene editor, dan inspector.

## 5.2 Main Application Layout

Struktur utama:

1. Top Bar.
2. Left Sidebar.
3. Center Workspace.
4. Right Inspector.
5. Bottom Panel.

Representasi layout:

```text
+------------------------------------------------------+
| TOP BAR                                              |
+--------------+----------------------+----------------+
|              |                      |                |
| PROJECT      |                      | INSPECTOR      |
| EXPLORER     |     WORKSPACE        |                |
|              |                      |                |
| ASSET        |     SCENE EDITOR      | PROPERTIES     |
| MANAGER      |                      |                |
|              |                      |                |
+--------------+----------------------+----------------+
| BOTTOM PANEL: OUTPUT / VALIDATION / LOGS             |
+------------------------------------------------------+
```

## 5.3 Top Bar

Top Bar menyediakan:

- Application menu.
- Project name.
- Undo.
- Redo.
- Save status.
- Preview / Play.
- Settings.
- Window controls.

Top Bar harus memiliki tampilan ringkas dan tidak mengambil terlalu banyak ruang vertikal.

## 5.4 Left Sidebar

Left Sidebar digunakan untuk navigasi project.

Bagian utama:

- Project Explorer.
- Asset Browser.
- Scene List.

Pengguna dapat membuka dan menutup kategori menggunakan expandable tree.

## 5.5 Center Workspace

Center Workspace merupakan area kerja utama.

Untuk Scene Editor, area ini menampilkan urutan Action Block secara vertikal.

Ketentuan:

- Block tersusun dari atas ke bawah.
- Pengguna dapat menambahkan block.
- Pengguna dapat mengubah urutan block.
- Pengguna dapat memilih block untuk mengedit properti.
- Pengguna dapat membuka dan menutup detail block.
- Urutan block harus mudah dipahami.

**Flowchart bukan tampilan utama Scene Editor.**

## 5.6 Right Inspector

Inspector menampilkan properti dari elemen yang sedang dipilih.

Contoh:

- Background Block: memilih background.
- Character Block: memilih karakter dan ekspresi.
- Dialogue Block: memilih speaker dan teks dialog.
- Audio Block: memilih BGM atau SFX.

Jika tidak ada elemen yang dipilih, Inspector dapat menampilkan informasi umum scene atau petunjuk penggunaan.

## 5.7 Bottom Panel

Bottom Panel digunakan untuk informasi pengembangan.

Tab yang disediakan:

- Output.
- Validation.
- Logs.

Panel dapat di-resize dan collapse.

## 5.8 Spacing Scale

Sistem spacing menggunakan kelipatan 4 px.

| Token | Value |
|---|---:|
| `space-1` | 4 px |
| `space-2` | 8 px |
| `space-3` | 12 px |
| `space-4` | 16 px |
| `space-5` | 20 px |
| `space-6` | 24 px |
| `space-8` | 32 px |

## 5.9 Radius Scale

| Token | Value |
|---|---:|
| `radius-sm` | 4 px |
| `radius-md` | 6 px |
| `radius-lg` | 8 px |
| `radius-xl` | 10 px |

Radius digunakan secara konsisten dan tidak boleh terlalu membulat.

## 5.10 Panel Management

Panel harus mendukung:

- Resize.
- Collapse.
- Expand.
- Restore default layout.

Workspace layout dapat dikembangkan agar pengguna dapat menyimpan preferensi panel.

Custom workspace persistence: **TBD**.

---

# 6. ICONOGRAPHY AND IMAGERY

## 6.1 Icon Philosophy

Icon digunakan untuk membantu pengguna mengenali fungsi komponen dengan cepat.

Gaya ikon:

- Minimal.
- Consistent stroke.
- Simple geometry.
- Recognizable.
- No unnecessary decoration.

## 6.2 Icon Library

Rekomendasi awal: Lucide Icons.

Ikon harus digunakan secara konsisten pada seluruh halaman.

## 6.3 Icon Sizes

| Size | Value | Usage |
|---|---:|---|
| Small | 14 px | Metadata dan compact controls |
| Medium | 16 px | Toolbar dan komponen |
| Large | 20 px | Navigation |
| Extra Large | 24 px | Empty states dan ilustrasi sederhana |

## 6.4 Icon Usage

Contoh kategori:

| Function | Icon Concept |
|---|---|
| Project | Folder |
| Scene | File Text |
| Background | Image |
| Character | User Round |
| Dialogue | Message Square |
| Narration | Book Open |
| BGM | Music |
| SFX | Volume 2 |
| Transition | Arrow Left Right |
| Choice | List Checks |
| Play | Play |
| Stop | Square |
| Settings | Settings |
| Validation | Shield Check |

Nama ikon final menyesuaikan ketersediaan library yang digunakan.

## 6.5 Asset Thumbnails

Asset Browser menggunakan thumbnail untuk:

- Backgrounds.
- Character sprites.
- Character expressions.
- UI assets.

Thumbnail harus mempertahankan rasio asli aset jika memungkinkan.

## 6.6 Empty State Illustrations

Empty state menggunakan ilustrasi sederhana atau ikon dengan deskripsi singkat.

Hindari ilustrasi besar yang tidak memiliki fungsi informatif.

---

# 7. CORE COMPONENTS

## 7.1 Component Philosophy

Seluruh komponen UI harus:

- Reusable.
- Consistent.
- Predictable.
- Accessible.
- Easy to maintain.

Komponen yang digunakan berulang harus memiliki spesifikasi visual dan interaksi yang sama.

## 7.2 Buttons

Jenis button:

| Variant | Usage |
|---|---|
| Primary | Aksi utama |
| Secondary | Aksi pendukung |
| Ghost | Aksi ringan |
| Danger | Aksi destruktif |
| Icon Button | Aksi berbasis ikon |

Button states:

- Default.
- Hover.
- Pressed.
- Focused.
- Disabled.
- Loading.

Button utama menggunakan indigo.

## 7.3 Input Fields

Input digunakan untuk:

- Project name.
- Dialogue text.
- Character name.
- Asset name.
- File path.
- Search.

Input states:

- Default.
- Focused.
- Filled.
- Error.
- Disabled.

## 7.4 Dropdown and Select

Dropdown digunakan ketika pengguna memilih salah satu nilai dari daftar.

Contoh:

- Character selection.
- Expression selection.
- Font selection.
- Audio selection.
- Transition type.

Dropdown harus mendukung pencarian jika daftar memiliki banyak pilihan.

## 7.5 Tabs

Tabs digunakan untuk membagi informasi dalam satu area.

Contoh:

- Output / Validation / Logs.
- Asset categories.
- Settings categories.

Tab aktif menggunakan indikator indigo.

## 7.6 Cards and Panels

Card digunakan untuk menampilkan informasi atau konfigurasi yang memiliki hubungan tertentu.

Card harus memiliki:

- Background.
- Optional border.
- Consistent padding.
- Clear hierarchy.

## 7.7 Action Block Component

Action Block merupakan komponen paling penting dalam Blanc Engine.

Block merepresentasikan sebuah tindakan yang dijalankan oleh runtime visual novel.

### Block Structure

```text
+------------------------------------------------+
| ICON   BLOCK TYPE                  STATUS      |
|                                                |
| Short configuration summary                    |
|                                                |
|                              DRAG HANDLE       |
+------------------------------------------------+
```

### Block Types

Jenis block awal:

| Block | Purpose | Detail Konfigurasi Utama |
|---|---|---|
| Background | Mengatur background scene | Memilih aset background dari library berdasarkan Asset ID |
| Character | Menampilkan karakter | Memilih karakter, Full Sprite per ekspresi, dan preset posisi layar (**Left**, **Center**, **Right**) |
| Dialogue | Menampilkan dialog | Memilih karakter pembicara dan memasukkan/menghubungkan teks dialog |
| Narration | Menampilkan narasi | Menampilkan teks narasi cerita tanpa pembicara |
| BGM | Memutar background music | Memilih audio latar, volume, dan pengaturan loop |
| SFX | Memutar sound effect | Memilih efek suara satu kali putar |
| Transition | Mengatur transisi | Mengatur efek visual (fade, dissolve) dan durasi |
| Wait | Memberikan jeda | Memberikan jeda waktu (detik) sebelum aksi berikutnya |
| Choice | Menampilkan pilihan pemain | Menentukan opsi pilihan teks dan target scene tujuan |
| Scene End | Mengakhiri scene | Menentukan aksi akhir: lanjut ke scene berikutnya, lompat ke scene tertentu, atau kembali ke Main Menu (*Return to Title*) |

Daftar block dapat berkembang sesuai kebutuhan produk.

### Block States

| State | Appearance |
|---|---|
| Default | Neutral surface |
| Hover | Slightly brighter surface |
| Selected | Indigo border and subtle accent background |
| Configured | Configured indicator |
| Incomplete | Warning indicator |
| Error | Error indicator |
| Disabled | Reduced visual emphasis |

### Block Interaction

Pengguna dapat:

- Select block.
- Edit block.
- Expand block.
- Collapse block.
- Reorder block.
- Duplicate block.
- Delete block.
- Add block.

Reordering dilakukan melalui drag-and-drop.

### Block Configuration

Konfigurasi utama dilakukan melalui Inspector.

Block dapat menampilkan ringkasan konfigurasi agar pengguna tetap memahami isi scene tanpa membuka setiap block.

## 7.8 Inspector Component

Inspector merupakan panel konfigurasi kontekstual.

Ketentuan:

- Berubah sesuai block yang dipilih.
- Menggunakan label yang jelas.
- Memiliki grouping berdasarkan fungsi.
- Tidak menampilkan informasi yang tidak relevan.
- Menampilkan validation feedback jika konfigurasi tidak valid.

## 7.9 Dialog and Modal

Modal digunakan untuk tindakan yang membutuhkan perhatian pengguna.

Contoh:

- Create Project.
- Import Assets.
- Delete Confirmation.
- Project Settings.
- Export Settings.

Modal harus memiliki judul, deskripsi, dan tombol tindakan yang jelas.

## 7.10 Notification and Toast

Toast digunakan untuk memberikan feedback singkat.

Contoh:

- Project saved.
- Asset imported.
- Scene updated.
- Export completed.
- Operation failed.

Notifikasi tidak boleh mengganggu pekerjaan pengguna secara berlebihan.

## 7.11 Context Menu

Context menu menyediakan tindakan tambahan berdasarkan elemen yang dipilih.

Contoh:

- Rename.
- Duplicate.
- Delete.
- Move.
- Open.
- Reveal in Folder.

---

# 8. SCREEN-LEVEL DESIGN GUIDELINES

## 8.1 Application Launch

Halaman awal menyediakan:

- Create New Project.
- Open Existing Project.
- Recent Projects.
- Project Location.

Tampilan harus sederhana dan langsung mengarahkan pengguna ke aktivitas utama.

## 8.2 Project Creation

Halaman pembuatan project menyediakan:

- Project Name.
- Project Location.
- Project File Manifest (`<NamaProject>.blanc`).
- Project Template, jika tersedia.
- Create Project.

Pengguna dapat memilih folder penyimpanan project. Sistem secara otomatis membuat struktur folder dan file manifest `<NamaProject>.blanc` yang berbasis format serialisasi JSON.

Sistem harus menampilkan informasi lokasi project secara jelas sebelum project dibuat.

## 8.3 Guided Workflow

Blanc Engine menggunakan guided workflow untuk membantu pengguna membuat visual novel.

Tahapan utama:

1. Project Setup.
2. Asset Management.
3. Story Management.
4. Scene Creation.
5. Preview and Testing.

Ketentuan desain:

- Pengguna dapat kembali ke tahap sebelumnya.
- Tahap yang telah selesai dapat dibuka kembali.
- Tahap berikutnya hanya dapat diakses setelah persyaratan tahap aktif terpenuhi.
- Status penyelesaian harus terlihat jelas.
- Pengguna tidak boleh kehilangan pekerjaan ketika berpindah tahap.

## 8.4 Asset Management

Asset Management digunakan untuk mengimpor, mempratinjau, dan mengorganisasi aset media proyek visual novel.

Format file resmi yang didukung engine:
- **Gambar:** PNG (`.png` — wajib untuk transparansi sprite), JPG/JPEG (`.jpg`, `.jpeg` — background), dan WebP (`.webp` — efisiensi kompresi).
- **Audio:** OGG Vorbis (`.ogg` — standar BGM gapless looping), WAV (`.wav` — efek suara SFX instan), dan MP3 (`.mp3`).
- **Font:** TTF (`.ttf`) dan OTF (`.otf`).

Kategori aset:

- Backgrounds.
- Characters.
- Character Expressions.
- BGM.
- SFX.
- Fonts.
- UI Assets.
- Other.

Layout yang disarankan:

- Category navigation (tree sidebar).
- Asset grid atau list view.
- Asset preview (thumbnail resolusi adaptif untuk gambar, pemutar audio mini untuk BGM/SFX, tester teks untuk font).
- Asset details & inspector.

Pengguna harus dapat melakukan pencarian dan pengelompokan aset. Setiap aset memiliki **Asset ID** unik yang dipetakan di file manifest `<NamaProject>.blanc`, sehingga penggantian nama file aset tidak merusak scene yang sedang menggunakannya.

## 8.5 Story Management

Halaman Story Management digunakan untuk memasukkan dan mengelola cerita secara visual maupun mengimpor naskah dari aplikasi penulisan eksternal.

Fitur:

- **Import Story:** Mendukung impor langsung dari format teks polos (`.txt`), Markdown (`.md`), dan format naskah skenario standar industri **Fountain (`.fountain`)** yang kompatibel dengan Scrivener, Highland, dan Fade In.
- **Story Parsing:** Otomatis memetakan heading bab/adegan, nama pembicara, ekspresi dalam kurung, dan dialog.
- **Internal Storage:** Naskah disimpan dalam format `.bstory` internal di dalam folder `Story/Chapters/`.
- **Dialogue & Narration Organization:** Mengatur dialog per karakter dan narasi cerita.
- **Story Preview:** Pratinjau teks cerita sebelum dihubungkan ke Action Block di Scene Editor.

## 8.6 Scene Editor

Scene Editor merupakan halaman kerja utama Blanc Engine. Setiap scene disimpan sebagai file adegan berformat **`.bscene`** di dalam subfolder chapter masing-masing (`Scenes/Chapter_XX/`).

Struktur:

- Left Project Explorer (menampilkan tree Chapter dan Scene `.bscene`).
- Center Block Editor (kanvas vertikal Action Blocks).
- Right Inspector (panel konfigurasi detail elemen terpilih).
- Bottom Output Panel.

Ketentuan:

- Scene ditampilkan sebagai daftar vertikal (bukan flowchart).
- Setiap block memiliki tipe aksi dan identitas visual yang jelas.
- Block dapat diurutkan melalui drag-and-drop.
- Block dapat dikonfigurasi melalui Inspector (posisi karakter Left/Center/Right, pemilihan ekspresi Full Sprite, dll.).
- Pengguna dapat menambahkan block kapan saja.
- Validation status dapat terlihat tanpa mengganggu editing.

## 8.7 Game Preview Window (Child Process Architecture)

Game Preview merupakan jendela mandiri yang dijalankan sebagai **Child Process** terpisah (`blanc_runtime`) dari Editor Window.

Ketika pengguna menekan tombol Play, Editor mengeksekusi binary `blanc_runtime` dengan argumen project dan scene terkait.

Keuntungan arsitektur Child Process:
1. **Isolasi Memori & Stabilitas:** Jika terjadi crash pada gameplay visual novel (misalnya aset rusak atau kesalahan logika branching), Editor tetap hidup dan data pekerjaan pengguna 100% aman.
2. **Kinerja Maksimal:** Runtime memiliki siklus render dan audio loop independen tanpa beban antarmuka editor yang berat.
3. **Kesiapan Ekspor:** Binary `blanc_runtime` merupakan binary yang sama persis dengan yang dikemas saat ekspor game mandiri.

### Runtime Window Requirements

- Memiliki jendela mandiri (Child Process).
- Memiliki ukuran tampilan standar default `1920 × 1080` dengan dukungan windowed mode dan fullscreen.
- Menampilkan visual novel berdasarkan data project (`<NamaProject>.blanc`, `.bscene`).
- Mendukung interaksi pemain penuh (keyboard, mouse).
- Dapat dihentikan kapan saja melalui tombol Stop pada editor.
- Tidak mengubah data project secara tidak sengaja.

### Playback Controls

| Control | Function |
|---|---|
| Play | Menjalankan runtime |
| Pause | Menjeda gameplay |
| Resume | Melanjutkan gameplay |
| Stop | Menghentikan runtime |
| Restart | Menjalankan ulang runtime |

### Play Entry Points

Blanc Engine direncanakan menyediakan:

- Play From Beginning.
- Play From Scene.

Play From Beginning menjalankan project dari awal.

Play From Scene menjalankan project dari scene yang sedang dipilih.

## 8.8 Built-in System Screens Design

Blanc Engine membedakan adegan cerita (*Story Scenes*) dan layar fungsionalitas sistem (*System Screens*). Layar sistem tidak perlu disusun manual menggunakan Action Blocks, melainkan disediakan secara terpadu oleh Runtime dan dikustomisasi visualnya oleh kreator melalui panel *Project Settings / System Screens*.

### 1. Main Menu (Title Screen)
- **Komponen Utama:**
  - Background (gambar atau video latar).
  - Game Title / Logo brand.
  - Menu Actions: *Start Game (New Game)*, *Load Game*, *Settings / Preferences*, dan *Quit*.
  - Title BGM (musik latar judul).
- **Karakter Desain:** Minimalis, terpusat atau rata kiri, tombol menggunakan kontras tinggi dengan state hover halus.

### 2. In-Game Pause Menu
- **Akses:** Dipanggil melalui tombol `Esc` atau ikon quick menu saat gameplay cerita berlangsung.
- **Komponen:**
  - Overlay semi-transparan (`surface-elevated` dengan opacity).
  - Quick Actions: *Resume*, *Save*, *Load*, *History / Backlog*, *Settings*, dan *Return to Title*.

### 3. Save & Load Screen
- **Komponen:**
  - Grid atau list slot penyimpanan (minimal 6–12 slot per halaman).
  - Informasi per slot: Screenshot thumbnail adegan saat disimpan, nama Chapter/Scene, timestamp penyimpanan, dan playtime.
  - Opsi tindakan: *Save*, *Load*, dan *Delete Slot*.

### 4. Settings / Preferences Screen
- **Komponen:**
  - Slider volume audio terpisah: *Master Volume*, *BGM Volume*, dan *SFX Volume*.
  - Pengaturan teks: *Text Display Speed* (slider) dan *Auto-Forward Delay*.
  - Tampilan layar: Toggle *Fullscreen* vs *Windowed*, dan pemilih resolusi windowed.

### 5. Non-Gameplay Screens Masa Depan (P1 & P2)
- **Fase P1:** *Dialogue Backlog Screen* (riwayat teks bergulir), *Splash Screen / Content Disclaimer*, dan *Credits Roll Screen*.
- **Fase P2:** *CG Gallery Screen* (koleksi gambar terbuka) dan *Music Room* (pemutar BGM).

## 8.9 Editor and Runtime Separation

Editor dan runtime harus memiliki tanggung jawab yang terpisah.

Editor berfungsi untuk:

- Mengelola project.
- Mengubah scene.
- Mengatur asset.
- Mengedit konfigurasi project dan System Screens.
- Menjalankan dan mengontrol runtime.

Runtime berfungsi untuk:

- Membaca data project (`<NamaProject>.blanc`).
- Menjalankan System Screens (Main Menu, Settings, Save/Load).
- Menjalankan urutan action block pada Story Scenes.
- Merender visual novel.
- Memutar audio.
- Menangani input pemain.
- Mengelola state gameplay.

Keduanya harus menggunakan representasi data project yang konsisten.

## 8.10 Settings

Settings menyediakan konfigurasi aplikasi dan project.

Application Settings:

- Theme (Dark / Light / System).
- Appearance.
- Workspace preferences.
- General preferences.

Project Settings:

- Project metadata (nama, versi, author).
- File konfigurasi disimpan di `Settings/system_screens.bconfig`.
- System Screens configuration (Title background, Title BGM, logo, starting scene).
- Runtime configuration (default resolution 1920x1080, text speed default, typewriter speed).
- UI theme visual novel.

Pengaturan lanjutan dapat ditambahkan pada versi berikutnya.

---

# 9. INTERACTION AND MOTION DESIGN

## 9.1 Interaction Philosophy

Interaksi harus memberikan feedback yang jelas dan tidak membingungkan.

Setiap tindakan pengguna harus menghasilkan perubahan visual atau feedback yang sesuai.

## 9.2 Mouse Interaction

Interaksi utama:

- Click to select.
- Double click to open.
- Right click for context menu.
- Drag and drop.
- Scroll.
- Resize panels.

## 9.3 Keyboard Interaction and Gameplay Controls

### Editor Shortcuts

| Shortcut | Action |
|---|---|
| Ctrl + S | Save Project |
| Ctrl + Z | Undo |
| Ctrl + Shift + Z | Redo |
| Ctrl + C | Copy Block / Element |
| Ctrl + V | Paste Block / Element |
| Ctrl + D | Duplicate Selected Block |
| Delete | Delete Selected Element |
| Ctrl + P | Play Preview from Beginning |
| Ctrl + Shift + P | Play Preview from Selected Scene |
| Escape | Close Dialog / Cancel Action |

### Runtime Gameplay Controls (Visual Novel Navigation)

| Input / Shortcut | Gameplay Action |
|---|---|
| Space / Left Click / Enter | Melanjutkan dialog ke baris berikutnya (*Next Dialogue*) |
| Ctrl (Hold) | Mempercepat jalannya teks dialog (*Fast Forward / Skip Mode*) |
| H | Menyembunyikan Dialogue Box (*Hide UI*) untuk melihat full visual |
| Mouse Scroll Up | Membuka riwayat percakapan (*Backlog / Dialogue History*) |
| Escape | Membuka layar jeda (*In-Game Pause Menu*) |
| F5 | Menyimpan cepat (*Quick Save*) |
| F8 / F9 | Memuat simpanan cepat (*Quick Load*) |

## 9.4 Drag and Drop

Drag-and-drop digunakan untuk:

- Mengubah urutan Action Block.
- Mengimpor asset.
- Memindahkan asset.
- Mengatur urutan elemen yang mendukung.

Drag-and-drop harus menampilkan indikator posisi tujuan.

## 9.5 Motion Guidelines

Animasi antarmuka harus minimal.

Rekomendasi durasi:

| Interaction | Duration |
|---|---:|
| Hover transition | 100–150 ms |
| Button feedback | 100–150 ms |
| Dropdown | 120–180 ms |
| Panel expand/collapse | 150–200 ms |
| Modal transition | 150–200 ms |

Motion tidak boleh mengganggu responsivitas aplikasi.

## 9.6 Undo and Redo

Perubahan yang relevan pada editor harus dapat dibatalkan dan dikembalikan.

Undo/Redo harus memberikan feedback yang konsisten.

Cakupan history dan batas jumlah perubahan: **TBD**.

## 9.7 Autosave

Autosave direncanakan untuk melindungi pekerjaan pengguna.

Desain status:

- Saving.
- Saved.
- Unsaved Changes.
- Save Failed.

Pengguna harus dapat mengetahui apakah perubahan sudah tersimpan.

---

# 10. STATES AND ERROR DESIGN

## 10.1 Component States

Komponen interaktif minimal memiliki:

- Default.
- Hover.
- Focus.
- Active.
- Selected.
- Disabled.
- Loading.
- Error.

Tidak semua state harus diterapkan pada setiap jenis komponen.

## 10.2 Empty States

Empty state digunakan ketika halaman belum memiliki data.

Contoh:

- No Projects.
- No Assets.
- No Characters.
- No Scenes.
- No Action Blocks.

Empty state harus memberikan penjelasan singkat dan tindakan berikutnya.

## 10.3 Loading States

Loading state digunakan untuk operasi seperti:

- Opening project.
- Importing assets.
- Loading scene.
- Preparing runtime.
- Exporting game.

Gunakan indikator progress jika estimasi proses tersedia.

## 10.4 Validation States

Validation digunakan untuk menunjukkan masalah konfigurasi.

Contoh:

- Missing background.
- Missing character asset.
- Empty dialogue.
- Invalid asset reference.
- Missing scene destination.

Validation harus menjelaskan:

1. Apa masalahnya.
2. Di mana masalah terjadi.
3. Bagaimana pengguna dapat memperbaikinya.

## 10.5 Error Handling

Error message harus:

- Menggunakan bahasa yang mudah dipahami.
- Menjelaskan penyebab jika diketahui.
- Menjelaskan dampak error.
- Memberikan solusi jika memungkinkan.

Hindari menampilkan pesan teknis mentah sebagai satu-satunya penjelasan kepada pengguna.

Informasi teknis tambahan dapat ditampilkan pada Output atau Logs.

## 10.6 Confirmation Dialog

Konfirmasi diperlukan untuk tindakan destruktif atau tindakan yang berisiko menghilangkan data.

Contoh:

- Delete Project.
- Delete Scene.
- Delete Asset.
- Discard Unsaved Changes.

Tindakan yang dapat dibatalkan sebaiknya tidak memerlukan konfirmasi berlebihan.

---

# 11. ACCESSIBILITY

## 11.1 Accessibility Goals

Blanc Engine harus dapat digunakan oleh sebanyak mungkin pengguna.

Desain harus mempertimbangkan:

- Text readability.
- Color contrast.
- Keyboard navigation.
- Focus visibility.
- Clear labeling.
- Consistent interaction.

## 11.2 Contrast

Teks dan komponen harus memiliki kontras yang memadai terhadap background.

Jangan hanya mengandalkan warna untuk menyampaikan status.

Contohnya, error harus menggunakan ikon atau pesan tambahan, bukan hanya border merah.

## 11.3 Keyboard Accessibility

Komponen penting harus dapat diakses menggunakan keyboard.

Focus indicator harus terlihat jelas.

## 11.4 Text Scaling

Ukuran teks dan layout harus mempertimbangkan kemungkinan pengaturan skala tampilan sistem operasi.

## 11.5 Accessibility Validation

Pemeriksaan accessibility dilakukan selama pengembangan dan pengujian.

Target kepatuhan terperinci: **TBD**.

---

# 12. RESPONSIVE DESKTOP DESIGN

## 12.1 Desktop-First Approach

Blanc Engine merupakan aplikasi desktop.

Desain dioptimalkan untuk penggunaan mouse dan keyboard dengan area kerja yang cukup luas.

## 12.2 Window Resizing

Layout harus dapat beradaptasi ketika ukuran jendela berubah.

Panel harus memiliki batas ukuran minimum dan maksimum yang masuk akal.

## 12.3 Minimum Window Size

Minimum window size: **TBD**.

Nilai final ditentukan melalui pengujian pada berbagai resolusi layar.

## 12.4 Panel Behavior

Ketika ruang kerja terbatas:

- Sidebar dapat di-collapse.
- Inspector dapat di-collapse.
- Bottom panel dapat disembunyikan.
- Center Workspace mendapatkan prioritas ruang.

## 12.5 Display Scaling

UI harus tetap dapat digunakan pada berbagai pengaturan display scaling.

---

# 13. THEME AND APPEARANCE MANAGEMENT

## 13.1 Theme Options

Blanc Engine menyediakan:

- Dark.
- Light.
- System Default.

## 13.2 Default Theme

Default theme adalah Dark.

## 13.3 Theme Consistency

Semua komponen harus menggunakan design tokens.

Warna tidak boleh ditentukan secara acak pada masing-masing komponen.

## 13.4 Theme Switching

Pergantian tema harus diterapkan secara konsisten pada seluruh antarmuka aplikasi.

Pengguna tidak perlu membuka ulang project hanya untuk mengubah tema.

## 13.5 Future Customization

Kemungkinan pengembangan:

- Custom accent color.
- Additional themes.
- Workspace presets.

Fitur tersebut bukan prioritas MVP.

---

# 14. TECHNICAL IMPLEMENTATION GUIDELINES

## 14.1 Technology

Blanc Engine menggunakan:

- Programming Language: Rust (idiomatic Rust).
- Architecture: **Cargo Workspace** modular multi-crate.
- UI Framework: Iced.
- Application Type: Desktop Application.

## 14.2 Design Implementation Principles

Implementasi UI harus mengikuti prinsip:

1. Component reusability.
2. Consistent design tokens.
3. Clear component responsibilities.
4. Maintainable styling.
5. Separation of UI and business logic.
6. Predictable state management.

## 14.3 Workspace Architecture & Design Tokens

Arsitektur kode menggunakan Cargo Workspace untuk memisahkan logika inti, editor desktop, dan game runtime:

```text
crates/
├── blanc_core/                  <-- Data models, logic, serialisasi, tokens dasar
│   ├── src/
│   │   ├── project.rs           <-- Struktur .blanc
│   │   ├── scene.rs             <-- Struktur .bscene & ActionBlock
│   │   ├── story.rs             <-- Parser TXT, Markdown, Fountain
│   │   ├── asset.rs             <-- Asset ID mapping
│   │   └── theme/               <-- Nilai token warna, typography, spacing
│   └── Cargo.toml
│
├── blanc_editor/                <-- Aplikasi GUI Editor utama (Iced)
│   ├── src/
│   │   ├── main.rs
│   │   ├── app.rs
│   │   ├── theme/               <-- Iced styling implementations
│   │   ├── components/          <-- Action blocks, buttons, inspector, dll.
│   │   ├── layouts/             <-- Multi-panel workspace layout
│   │   └── screens/             <-- Welcome, Asset, Story, Scene Editor, Settings
│   └── Cargo.toml
│
└── blanc_runtime/               <-- Game Preview & Standalone Player (Iced)
    ├── src/
    │   ├── main.rs
    │   ├── player.rs
    │   ├── system_screens/      <-- Main Menu, Pause, Settings, Save/Load
    │   └── renderer/            <-- Dialogue box, sprite & background renderer
    └── Cargo.toml
```

Struktur tersebut memisahkan kode editor dari game player sehingga ekspor game mandiri dapat dilakukan secara bersih dan optimal.

## 14.4 Component Architecture

Komponen UI harus dipisahkan berdasarkan tanggung jawab.

Contoh:

- Button tidak menangani penyimpanan project.
- Action Block tidak menangani seluruh proses runtime.
- Inspector tidak memiliki tanggung jawab mengelola seluruh asset.
- Theme system menjadi sumber utama styling.

## 14.5 State Management

State UI harus dikelola secara terstruktur.

Pisahkan:

- Application state.
- Project state.
- Editor state.
- Selection state.
- Runtime state.
- UI state.

Perubahan state harus menghasilkan tampilan yang konsisten.

## 14.6 Editor and Runtime Architecture

Editor Window dan Game Runtime Window merupakan dua bagian dengan tanggung jawab berbeda.

Desain antarmuka harus mendukung pemisahan tersebut tanpa membuat data project menjadi tidak konsisten.

Detail komunikasi, lifecycle, dan sinkronisasi runtime mengacu pada SRS dan technical architecture.

## 14.7 Performance

UI harus tetap responsif ketika:

- Project memiliki banyak asset.
- Scene memiliki banyak Action Block.
- Pengguna berpindah antar-scene.
- Inspector memuat konfigurasi kompleks.
- Runtime sedang berjalan.

Pengelolaan thumbnail dan rendering harus mempertimbangkan penggunaan memori.

## 14.8 Platform Compatibility

Target sistem operasi: **TBD**.

Implementasi visual harus mempertimbangkan perbedaan window management dan display scaling pada platform yang didukung.

---

# 15. DESIGN CONSISTENCY AND QUALITY CONTROL

## 15.1 Consistency Rules

- Gunakan design tokens yang telah ditetapkan.
- Gunakan komponen reusable.
- Gunakan ukuran spacing yang konsisten.
- Gunakan typography scale yang sama.
- Gunakan semantic colors sesuai fungsinya.
- Hindari styling khusus tanpa alasan yang jelas.

## 15.2 Design Review

Setiap fitur baru harus diperiksa berdasarkan:

- Visual consistency.
- Interaction consistency.
- Accessibility.
- Usability.
- Performance.
- Compatibility dengan existing components.

## 15.3 UI Testing

Pengujian UI mencakup:

- Component states.
- Layout resizing.
- Theme switching.
- Keyboard interactions.
- Mouse interactions.
- Empty states.
- Error states.
- Runtime window behavior.

## 15.4 Design Acceptance Criteria

Design System dapat dianggap diterapkan dengan baik ketika:

1. Seluruh halaman utama menggunakan sistem warna yang konsisten.
2. Typography mengikuti hierarchy yang ditentukan.
3. Komponen utama menggunakan design tokens.
4. Action Block memiliki tampilan dan interaksi konsisten.
5. Editor memiliki struktur multi-panel.
6. Game Runtime berjalan pada jendela terpisah.
7. Layout dapat beradaptasi terhadap perubahan ukuran jendela.
8. Dark dan Light theme memiliki konsistensi visual.
9. Feedback dan validation mudah dipahami.
10. UI tidak mengharuskan pengguna memahami kode untuk membuat visual novel.

---

# 16. OPEN DESIGN DECISIONS

Bagian ini mencatat hal-hal yang belum ditetapkan secara final.

| ID | Decision | Status |
|---|---|---|
| DD-001 | Final logo and brand assets | TBD |
| DD-002 | Supported operating systems | TBD |
| DD-003 | Minimum window resolution | TBD |
| DD-004 | Final icon library integration | TBD |
| DD-005 | Custom workspace layout persistence | TBD |
| DD-006 | Custom accent color support | Future |
| DD-007 | Runtime window resolution options | TBD |
| DD-008 | Live Reload during runtime | Future |
| DD-009 | Advanced accessibility targets | TBD |
| DD-010 | Additional workspace presets | Future |

Keputusan baru harus diperbarui pada dokumen ini agar implementasi tetap konsisten.

---

# 17. APPENDIX

## 17.1 Design Glossary

| Term | Description |
|---|---|
| Design Token | Nilai desain terpusat |
| Accent | Warna penekanan utama |
| Surface | Area atau background komponen |
| Inspector | Panel konfigurasi elemen |
| Action Block | Unit tindakan dalam Scene Editor |
| Workspace | Area kerja utama aplikasi |
| Runtime | Sistem yang menjalankan visual novel |
| Preview | Jendela untuk menguji hasil game |
| Semantic Color | Warna untuk menunjukkan status |
| Guided Workflow | Alur kerja bertahap |

## 17.2 Related Documents

- `SRS.md` — Software Requirements Specification.
- `PRD.md` — Product Requirements Document.
- `Design_System.md` — UI/UX Design Specification.

## 17.3 Document Change Log

| Version | Date | Changes |
|---|---|---|
| 0.1 | 12 November 2026 | Initial Design System draft |

---

**END OF DOCUMENT**  
**BLANC ENGINE — DESIGN SYSTEM v0.1**
