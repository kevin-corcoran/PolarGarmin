let SessionLoad = 1
let s:so_save = &g:so | let s:siso_save = &g:siso | setg so=0 siso=0 | setl so=-1 siso=-1
let v:this_session=expand("<sfile>:p")
silent only
silent tabonly
cd ~/Projects/Tauri/garmin-app/src-tauri
if expand('%') == '' && !&modified && line('$') <= 1 && getline(1) == ''
  let s:wipebuf = bufnr('%')
endif
let s:shortmess_save = &shortmess
if &shortmess =~ 'A'
  set shortmess=aoOA
else
  set shortmess=aoO
endif
badd +1 src/database.rs
badd +1 src/lib.rs
badd +1 src/main.rs
badd +1 ~/Projects/Tauri/garmin-app/src/App.vue
badd +3 ~/Projects/Tauri/garmin-app/src/components/RunningChart.vue
badd +13 ~/Projects/Tauri/garmin-app/index.html
badd +0 ~/Projects/Tauri/GarminLab/src/App.vue
argglobal
%argdel
$argadd src/database.rs
$argadd src/lib.rs
$argadd src/main.rs
edit ~/Projects/Tauri/garmin-app/src/App.vue
let s:save_splitbelow = &splitbelow
let s:save_splitright = &splitright
set splitbelow splitright
wincmd _ | wincmd |
vsplit
1wincmd h
wincmd w
let &splitbelow = s:save_splitbelow
let &splitright = s:save_splitright
wincmd t
let s:save_winminheight = &winminheight
let s:save_winminwidth = &winminwidth
set winminheight=0
set winheight=1
set winminwidth=0
set winwidth=1
wincmd =
argglobal
if bufexists(fnamemodify("~/Projects/Tauri/garmin-app/src/App.vue", ":p")) | buffer ~/Projects/Tauri/garmin-app/src/App.vue | else | edit ~/Projects/Tauri/garmin-app/src/App.vue | endif
if &buftype ==# 'terminal'
  silent file ~/Projects/Tauri/garmin-app/src/App.vue
endif
balt ~/Projects/Tauri/GarminLab/src/App.vue
setlocal foldmethod=manual
setlocal foldexpr=0
setlocal foldmarker={{{,}}}
setlocal foldignore=#
setlocal foldlevel=0
setlocal foldminlines=1
setlocal foldnestmax=20
setlocal foldenable
silent! normal! zE
let &fdl = &fdl
let s:l = 1 - ((0 * winheight(0) + 23) / 46)
if s:l < 1 | let s:l = 1 | endif
keepjumps exe s:l
normal! zt
keepjumps 1
normal! 0
wincmd w
argglobal
if bufexists(fnamemodify("~/Projects/Tauri/GarminLab/src/App.vue", ":p")) | buffer ~/Projects/Tauri/GarminLab/src/App.vue | else | edit ~/Projects/Tauri/GarminLab/src/App.vue | endif
if &buftype ==# 'terminal'
  silent file ~/Projects/Tauri/GarminLab/src/App.vue
endif
balt ~/Projects/Tauri/garmin-app/src/App.vue
setlocal foldmethod=manual
setlocal foldexpr=0
setlocal foldmarker={{{,}}}
setlocal foldignore=#
setlocal foldlevel=0
setlocal foldminlines=1
setlocal foldnestmax=20
setlocal foldenable
silent! normal! zE
let &fdl = &fdl
let s:l = 1 - ((0 * winheight(0) + 23) / 46)
if s:l < 1 | let s:l = 1 | endif
keepjumps exe s:l
normal! zt
keepjumps 1
normal! 0
wincmd w
wincmd =
tabnext 1
if exists('s:wipebuf') && len(win_findbuf(s:wipebuf)) == 0 && getbufvar(s:wipebuf, '&buftype') isnot# 'terminal'
  silent exe 'bwipe ' . s:wipebuf
endif
unlet! s:wipebuf
set winheight=1 winwidth=20
let &shortmess = s:shortmess_save
let &winminheight = s:save_winminheight
let &winminwidth = s:save_winminwidth
let s:sx = expand("<sfile>:p:r")."x.vim"
if filereadable(s:sx)
  exe "source " . fnameescape(s:sx)
endif
let &g:so = s:so_save | let &g:siso = s:siso_save
set hlsearch
nohlsearch
doautoall SessionLoadPost
unlet SessionLoad
" vim: set ft=vim :
