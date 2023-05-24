import os
import subprocess

from pathlib import Path

from obsidianhtml.modules.base_classes import ObsidianHtmlModule


class ObsidianHtmlRustExampleModule(ObsidianHtmlModule):
    @property
    def requires(self):
        return tuple()

    @property
    def provides(self):
        return tuple()

    @property
    def alters(self):
        return tuple()

    def run(self):
        print("[Module] Hello!")
        self.run_process("src/hello")
        print("[Module] Bye!")

    def run_process(self, path):
        path = self.get_process_path(path)
        process_res = subprocess.run([path], stdout=subprocess.PIPE)
        print(process_res.stdout.decode("utf-8"), end="")

    def get_process_path(self, path_str):
        """Use path relative to the file from which you make the call"""
        python_file_folder = Path(__file__).parent.resolve()
        requested_file = python_file_folder.joinpath(path_str)
        if requested_file.exists() is False:
            raise Exception(
                f"Requested file {path_str} from {__file__}, resulting in path {requested_file}, was not found."
            )
        return requested_file.as_posix()
