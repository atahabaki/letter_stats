# letter_stats

These days, everyone spends their time in front of a computer. And guess what 
component of a computer is used the most? Yes, the keyboard.

Qwerty is developed in 1873. We need a better layout. And hey, everyone using 
their keyboard on different places, such as programming, scripting, text 
writing, et cetera. So, I think everyone needs a layout based on their usage 
statics.

At least, we need to create/design new layouts for different areas. For 
example, python users may use the layout for python programming language.

Anyway, this program takes a file, reads it, and generates the usage ratio of 
characters, and the ratio of next character.

```bash
./letter_stats -f <FILE>
```

<details>
<summary>Statistics of <code>examples/example.rs</code></summary>
<pre>
{
    'e': LetterStat {
        count: 2,
        leading_stat: {
            't': 1,
            'r': 1,
        },
    },
    '{': LetterStat {
        count: 1,
        leading_stat: {},
    },
    't': LetterStat {
        count: 5,
        leading_stat: {
            'e': 1,
            'a': 1,
            'l': 1,
            's': 1,
        },
    },
    'r': LetterStat {
        count: 2,
        leading_stat: {
            '_': 1,
            'i': 1,
        },
    },
    'm': LetterStat {
        count: 1,
        leading_stat: {
            'a': 1,
        },
    },
    'a': LetterStat {
        count: 2,
        leading_stat: {
            'i': 1,
            't': 1,
        },
    },
    ')': LetterStat {
        count: 2,
        leading_stat: {
            ';': 1,
        },
    },
    '(': LetterStat {
        count: 2,
        leading_stat: {
            '"': 1,
            ')': 1,
        },
    },
    'p': LetterStat {
        count: 1,
        leading_stat: {
            'r': 1,
        },
    },
    'i': LetterStat {
        count: 2,
        leading_stat: {
            'n': 2,
        },
    },
    'l': LetterStat {
        count: 2,
        leading_stat: {
            'e': 1,
            'n': 1,
        },
    },
    '"': LetterStat {
        count: 2,
        leading_stat: {
            ')': 1,
            'l': 1,
        },
    },
    'f': LetterStat {
        count: 1,
        leading_stat: {
            'n': 1,
        },
    },
    '_': LetterStat {
        count: 1,
        leading_stat: {
            's': 1,
        },
    },
    '}': LetterStat {
        count: 1,
        leading_stat: {},
    },
    ';': LetterStat {
        count: 1,
        leading_stat: {},
    },
    's': LetterStat {
        count: 2,
        leading_stat: {
            't': 1,
            '"': 1,
        },
    },
    'n': LetterStat {
        count: 4,
        leading_stat: {
            '!': 1,
            '(': 1,
            't': 1,
        },
    },
    '!': LetterStat {
        count: 1,
        leading_stat: {
            '(': 1,
        },
    },
}
</pre>
</details>
