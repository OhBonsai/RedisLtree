import os


def execute_commands_file(file_name, redis_client):
    cwd = os.getcwd()
    file_name = os.path.join(cwd, "./tests/files", file_name)

    for line in open(file_name, "r").readlines():
        if line.strip().startswith("#"):
            continue

        args = [arg.strip() for arg in line.split("||")]
        redis_client.execute_command(*args)
