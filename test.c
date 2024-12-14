int read(int fd, char *buf, int count);
int write(int fd, const char *buf, int count);

int main() {
    write(1, "Hello, world!\n", 13);
    return 0;
}
