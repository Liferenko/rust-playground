let SessionLoad = 1
let s:so_save = &g:so | let s:siso_save = &g:siso | setg so=0 siso=0 | setl so=-1 siso=-1
let v:this_session=expand("<sfile>:p")
silent only
silent tabonly
cd ~/projects/rust-wasm/cloud_sync
if expand('%') == '' && !&modified && line('$') <= 1 && getline(1) == ''
  let s:wipebuf = bufnr('%')
endif
let s:shortmess_save = &shortmess
if &shortmess =~ 'A'
  set shortmess=aoOA
else
  set shortmess=aoO
endif
badd +1 src
badd +13 src/main.rs
badd +2 term://~/projects/rust-wasm/cloud_sync//26925:mv\ src/test/test_main.rs\ src/main_tests.rs
badd +3 src/main_tests.rs
badd +8 term://~/projects/rust-wasm/cloud_sync//27364:cargo\ test
badd +13 term://~/projects/rust-wasm/cloud_sync//28149:cargo\ test
badd +1 term://~/projects/rust-wasm/cloud_sync//28813:cargo\ test
badd +11 Cargo.toml
badd +1 README.md
badd +55 ~/projects/rust-wasm/README.md
badd +1 ~/pets/rust/my-redis
badd +2 ~/pets/rust/my-redis/examples/hello-redis.rs
badd +9 ~/pets/rust/my-redis/Cargo.toml
badd +1 examples/main.rs
badd +28 examples/client.rs
badd +3 examples/client_test.rs
badd +0 ~/pets/rust/my-redis/src/main.rs
argglobal
%argdel
$argadd src
set stal=2
tabnew +setlocal\ bufhidden=wipe
tabrewind
edit src/main.rs
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
exe 'vert 1resize ' . ((&columns * 90 + 91) / 182)
exe 'vert 2resize ' . ((&columns * 91 + 91) / 182)
argglobal
balt examples/client.rs
setlocal fdm=manual
setlocal fde=0
setlocal fmr={{{,}}}
setlocal fdi=#
setlocal fdl=0
setlocal fml=1
setlocal fdn=20
setlocal fen
silent! normal! zE
let &fdl = &fdl
let s:l = 13 - ((12 * winheight(0) + 17) / 35)
if s:l < 1 | let s:l = 1 | endif
keepjumps exe s:l
normal! zt
keepjumps 13
normal! 034|
lcd ~/projects/rust-wasm/cloud_sync
wincmd w
argglobal
if bufexists(fnamemodify("~/pets/rust/my-redis/src/main.rs", ":p")) | buffer ~/pets/rust/my-redis/src/main.rs | else | edit ~/pets/rust/my-redis/src/main.rs | endif
if &buftype ==# 'terminal'
  silent file ~/pets/rust/my-redis/src/main.rs
endif
setlocal fdm=manual
setlocal fde=0
setlocal fmr={{{,}}}
setlocal fdi=#
setlocal fdl=0
setlocal fml=1
setlocal fdn=20
setlocal fen
silent! normal! zE
let &fdl = &fdl
let s:l = 3 - ((2 * winheight(0) + 17) / 35)
if s:l < 1 | let s:l = 1 | endif
keepjumps exe s:l
normal! zt
keepjumps 3
normal! 0
lcd ~/projects/rust-wasm/cloud_sync
wincmd w
exe 'vert 1resize ' . ((&columns * 90 + 91) / 182)
exe 'vert 2resize ' . ((&columns * 91 + 91) / 182)
tabnext
edit ~/projects/rust-wasm/cloud_sync/README.md
argglobal
balt ~/projects/rust-wasm/cloud_sync/Cargo.toml
setlocal fdm=manual
setlocal fde=0
setlocal fmr={{{,}}}
setlocal fdi=#
setlocal fdl=0
setlocal fml=1
setlocal fdn=20
setlocal fen
silent! normal! zE
let &fdl = &fdl
let s:l = 28 - ((27 * winheight(0) + 17) / 35)
if s:l < 1 | let s:l = 1 | endif
keepjumps exe s:l
normal! zt
keepjumps 28
normal! 021|
lcd ~/projects/rust-wasm/cloud_sync
tabnext 1
set stal=1
if exists('s:wipebuf') && len(win_findbuf(s:wipebuf)) == 0 && getbufvar(s:wipebuf, '&buftype') isnot# 'terminal'
  silent exe 'bwipe ' . s:wipebuf
endif
unlet! s:wipebuf
set winheight=1 winwidth=20
let &shortmess = s:shortmess_save
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
