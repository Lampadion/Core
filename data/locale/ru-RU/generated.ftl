# ─────────────────────────────
# Скрипты / прилагательные
# ─────────────────────────────

-script-latin =
    { $gender ->
       *[masculine] латинский
        [feminine] латинская
        [neuter] латинское
        [plural] латинские
    }

-script-cyrillic =
    { $gender ->
       *[masculine] кириллический
        [feminine] кириллическая
        [neuter] кириллическое
        [plural] кириллические
    }


# ─────────────────────────────
# Регистр
# ─────────────────────────────

-case-capital =
    { $gender ->
        [masculine] прописной
        [feminine] прописная
        [neuter] прописное
       *[plural] прописные
    }

-case-lowercase =
    { $gender ->
        [masculine] строчной
        [feminine] строчная
        [neuter] строчное
       *[plural] строчные
    }


# ─────────────────────────────
# Существительные
# ─────────────────────────────

-noun-letter =
    { $case ->
        [nominative] буква
        [genitive] буквы
        [dative] букве
        [accusative] букву
        [instrumental] буквой
        [prepositional] букве
       *[default] буква
    }

-noun-symbol =
    { $case ->
        [nominative] символ
        [genitive] символа
        [dative] символу
        [accusative] символ
        [instrumental] символом
        [prepositional] символе
       *[default] символ
    }
