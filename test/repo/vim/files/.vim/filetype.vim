" global filetype.vim in the vim package

if exists("did_load_filetypes")
  finish
endif

augroup filetypedetect
  au! BufNewFile,BufRead .tmux.conf*,tmux.conf*     setf tmux
  au! BufNewFile,BufRead *.rs                       setf rust
augroup END
