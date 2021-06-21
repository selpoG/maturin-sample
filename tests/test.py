import unittest

from maturin_sample import hello


class SampleTestCase(unittest.TestCase):
    def test_hello(self):
        self.assertEqual(hello([]), 0)
        self.assertEqual(hello([5]), 1)
        self.assertEqual(hello([9, 1, 5, 2, 3]), 5)


if __name__ == "__main__":
    unittest.main()
