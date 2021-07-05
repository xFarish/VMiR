import sys, os, shutil

def find_exec(name: str) -> bool:
    return shutil.which(name)

def main():
    print('Locating `cargo`...')
    if not find_exec('cargo'):
        print('Could not locate `cargo`')
        print('Aborting...')
        sys.exit(1)

    print('> cargo build')
    os.system('cargo build')

    print('Run `cargo run` to run the project')

if __name__ == '__main__':
    main()