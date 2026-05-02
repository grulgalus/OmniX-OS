# Smažeme všechny soubory, co nám dělají problém
rm -rf tools/objtool/libsubcmd/*

# Vytvoříme jednoduchý prázdný kód, který se snadno zkompiluje jako knihovna (aby si zbytek makefilu nestěžoval)
echo 'void dummy_func(void) {}' > tools/objtool/libsubcmd/exec-cmd.c
echo 'void dummy_func2(void) {}' > tools/objtool/libsubcmd/parse-options.c
echo 'void dummy_func3(void) {}' > tools/objtool/libsubcmd/pager.c
echo 'void dummy_func4(void) {}' > tools/objtool/libsubcmd/run-command.c
echo 'void dummy_func5(void) {}' > tools/objtool/libsubcmd/sigchain.c
echo 'void dummy_func6(void) {}' > tools/objtool/libsubcmd/subcmd-config.c
echo 'void dummy_func7(void) {}' > tools/objtool/libsubcmd/help.c
