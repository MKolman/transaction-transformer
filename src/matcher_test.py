import unittest
from unittest import mock

import matcher


class TestStrDist(unittest.TestCase):
    def test_empty(self):
        self.assertEqual(matcher.str_dist("", ""), 0)

    def test_equal(self):
        test_str = "This is a test string 123"
        self.assertEqual(matcher.str_dist(test_str, test_str), 0)

    def test_rm(self):
        test_str = "This is a test string 123"
        self.assertEqual(matcher.str_dist("", test_str), len(test_str))
        self.assertEqual(matcher.str_dist(test_str, ""), len(test_str))

    def test_change(self):
        self.assertEqual(matcher.str_dist("effect", "affect"), 1)

    def test_multiple(self):
        self.assertEqual(matcher.str_dist("efecters", "affecter"), 3)


class TestMatch(unittest.TestCase):
    def test_empty(self):
        self.assertEqual(matcher.match("", ""), 1)

    def test_equal(self):
        test_str = "This is a test string 123"
        self.assertEqual(matcher.match(test_str, test_str), 1)

    def test_rm(self):
        test_str = "This is a test string 123"
        self.assertEqual(matcher.match("", test_str), 0)
        self.assertAlmostEqual(matcher.match(test_str, ""), 0)

    def test_change(self):
        self.assertEqual(matcher.match("effect", "affect"), 5 / 6)

    def test_multiple(self):
        self.assertEqual(matcher.match("efecters", "affecter"), 0.625)

    def test_full(self):
        self.assertGreater(
            matcher.match(
                "KOLMAN MAKS | SI56 6100 0002 6162 702 | REZERVNI SKLAD 12 2021",
                "KOLMAN MAKS | SI56 6100 0002 6162 702 | REDNE OBVEZNOSTI 12 2021",
            ),
            0.75,
        )

        self.assertLess(
            matcher.match(
                "VEGRIM d.o.o. | SI56 6100 0001 0696 634 | REDNE OBVEZNOSTI 12 2021",
                "VEGRIM D.O.O., LJUBLJANA   | SI56 1010 0003 7935 919 | REZERVNI SKLAD 12 2021",
            ),
            0.75,
        )


class TestAccountMatcher(unittest.TestCase):
    def test_full_match(self):
        acc_matcher = matcher.AccountMatcher({"test": "blah"})
        self.assertEqual(acc_matcher.find_match("test"), "blah")

    def test_good(self):
        acc_matcher = matcher.AccountMatcher({"test account": "blah"})
        self.assertEqual(acc_matcher.find_match("test accounts"), "blah")

    @mock.patch("matcher.input", mock.Mock(return_value="1"))
    @mock.patch("matcher.print")
    def test_pick_from_list(self, print_mock):
        def append_prompt(*args, end="\n"):
            nonlocal prompt
            prompt += " ".join(map(str, args)) + end

        prompt = ""
        print_mock.side_effect = append_prompt
        acc_matcher = matcher.AccountMatcher({"testy": "blah"})
        self.assertEqual(acc_matcher.find_match("testis"), "blah")
        self.assertIn("testis", prompt)
        self.assertIn("1. 66.7% (blah) testy", prompt)

    @mock.patch("matcher.input", mock.Mock(return_value="1"))
    @mock.patch("matcher.print")
    def test_cannot_pick_from_empty_lsit(self, print_mock):
        def append_prompt(*args, end="\n"):
            nonlocal prompt
            prompt += " ".join(map(str, args)) + end

        prompt = ""
        print_mock.side_effect = append_prompt
        acc_matcher = matcher.AccountMatcher({"asdf": "blah"})
        self.assertEqual(acc_matcher.find_match("tests"), "1")
        self.assertIn("enter a new match", prompt)
        self.assertNotIn("one of the numbers", prompt)
