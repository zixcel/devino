import importlib.util
from pathlib import Path
import tempfile
import unittest
import sys

module_path = Path(__file__).resolve().parents[1] / "src/setup/bert_base_uncase.py"
spec = importlib.util.spec_from_file_location("devino_model_configuration", module_path)
module = importlib.util.module_from_spec(spec)
sys.modules[spec.name] = module
spec.loader.exec_module(module)


class ModelConfigurationTests(unittest.TestCase):
    def values(self, folder):
        return {"MODEL_ID": "fixture/model", "MODEL_REVISION": "a" * 40,
                "MODEL_DEVICE": "CPU", "MODEL_OUTPUT": str(Path(folder) / "model.xml"),
                "MODEL_TEXT": "private-fixture-input"}

    def test_import_and_config_do_not_import_or_download_backends(self):
        self.assertNotIn("transformers", sys.modules)
        self.assertNotIn("openvino", sys.modules)
        with tempfile.TemporaryDirectory() as folder:
            config = module.ModelConfig.from_env(self.values(folder))
            self.assertNotIn("private-fixture-input", repr(config))
            self.assertEqual(config.device, "CPU")
            self.assertEqual(list(Path(folder).iterdir()), [])

    def test_required_settings_and_mutable_model_revisions_are_rejected(self):
        with tempfile.TemporaryDirectory() as folder:
            values = self.values(folder)
            for key in values:
                missing = {k: v for k, v in values.items() if k != key}
                with self.assertRaises(ValueError):
                    module.ModelConfig.from_env(missing)
            for revision in ["main", "v1", "a" * 39, "g" * 40]:
                with self.assertRaises(ValueError):
                    module.ModelConfig.from_env({**values, "MODEL_REVISION": revision})
            with self.assertRaises(ValueError):
                module.ModelConfig.from_env({**values, "MODEL_OUTPUT": "model.xml"})

    def test_existing_outputs_reject_before_backend_import_and_remain_intact(self):
        with tempfile.TemporaryDirectory() as folder:
            config = module.ModelConfig.from_env(self.values(folder))
            config.output.with_suffix(".bin").write_bytes(b"preserved fixture")
            with self.assertRaises(FileExistsError):
                module.convert(config)
            self.assertEqual(config.output.with_suffix(".bin").read_bytes(), b"preserved fixture")
            self.assertNotIn("transformers", sys.modules)


if __name__ == "__main__":
    unittest.main()
