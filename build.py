import sys, os, shutil

def info(line: int, file: str) -> None:
    print('\n> {}'.format(file))
    print('{} | ...'.format((str(line - 1))))
    print('{} | // Debug'.format((str(line))))
    print('{} | #[allow(dead_code)]'.format((str(line + 1))))
    print('{} | ...'.format((str(line + 2))))

def print_info() -> None:
    print('During production build, make sure to remove these: ')
    info(3, 'src/cpu.rs')
    info(23, 'src/cpu.rs')
    info(5, 'src/vm.rs')
    info(19, 'src/vm.rs')
    info(126, 'src/vm.rs')

def find_exec(name: str) -> bool:
    return shutil.which(name)

def build():
    print('Locating `cargo`...')
    if not find_exec('cargo'):
        print('Could not locate `cargo`')
        print('Aborting...')
        sys.exit(1)

    print('> cargo build')
    os.system('cargo build')

    print('Run `cargo run` to run the project')

def main():
    print_info()
    build()

if __name__ == '__main__':
    main()