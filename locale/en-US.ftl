# Postfixes

title-postfix =
    { $conjunction } { $postfix }

title-alt-postfix =
    { $conjunction } { $word-alt-separator } { $postfix }

title-postfix-multi =
    { $word-separator }{ $postfix-and } { $postfix }

title-alt-postfix-multi =
    { $word-separator }{ $postfix-and } { $word-alt-separator } { $postfix }


# Numbers

copy-number = [{ $number }]
index-number = { $number }


# Main title

title =
    { $before-title }
    { $script }
    { $before-type }
    { $case }
    { $type }
    { $after-type }
    { $script-additive }
    { $before-letter }
    { $letter }
    { $after-letter }
    { $second-name }
    { $copy-number }
    { $proxy-mark }
    { $after-title }
    { $postfix }


# Alternative title

title-alt =
    { $before-alt-title }
    { $alt-title-line-break }
    { $type }
    { $before-letter }
    { $letter }
    { $after-letter }
    { $second-name }
    { $proxy-mark }
    { $alt-title-line-break-after }
    { $after-alt-title }
    { $alt-postfix }


# Tag

tag =
    { $script-tag }
    { $type-tag }
    { $tag-script-additive }
    { $before-title }
    { $case }
    { $before-type }
    { $type-non-germanic }
    { $after-type }
    { $before-letter }
    { $letter }
    { $after-letter }
    { $second-name }
    { $copy-number }
    { $after-title }
    { $postfix }


# Hidden tag

hidden-tag =
    { $script-tag-hidden }
    { $type-tag-hidden }
    { $tag-hidden-script-additive }
    { $before-title }
    { $case }
    { $before-type }
    { $type-non-germanic }
    { $after-type }
    { $before-letter }
    { $hidden-letter }
    { $after-letter }
    { $second-name }
    { $copy-number }
    { $after-title }
    { $postfix }


# Additive title

additive-title =
    { $additional-before-title }
    { $script }
    { $additional-before-type }
    { $case }
    { $type-tag }
    { $additional-after-type }
    { $script-additive }
    { $additional-before-letter }
    { $additional-letter }
    { $additional-after-letter }
    { $additional-copy-number }
    { $proxy-mark }
    { $additional-after-title }


# Additive tag

additive-tag =
    { $additional-before-title }
    { $script-tag }
    { $additional-before-type }
    { $case }
    { $type-tag }
    { $additional-after-type }
    { $additional-before-letter }
    { $additional-letter }
    { $additional-after-letter }
    { $additional-copy-number }
    { $additional-after-title }


# Additive hidden tag

additive-hidden-tag =
    { $hidden-additional-before-title }
    { $script-hidden-tag }
    { $hidden-additional-before-type }
    { $hidden-case } { $hidden-type-tag }
    { $hidden-additional-after-type }
    { $hidden-additional-before-letter }
    { $additional-hidden-letter }
    { $hidden-additional-after-letter }
    { $additional-copy-number }
    { $hidden-additional-after-title }


# Additive Germanic tag

additive-germanic-tag =
    { $additional-before-title }
    { $additional-script }
    { $additional-before-type }
    { $additional-type }
    { $additional-after-type }
    { $additional-script-additive }
    { $additional-before-letter }
    { $additional-letter }
    { $additional-after-letter }
    { $additional-copy-number }
    { $additional-after-title }


# Additive Germanic hidden tag

additive-germanic-hidden-tag =
    { $hidden-additional-before-title }
    { $hidden-additional-script }
    { $hidden-additional-before-type }
    { $hidden-additional-type }
    { $hidden-additional-after-type }
    { $hidden-additional-script-additive }
    { $hidden-additional-before-letter }
    { $additional-hidden-letter }
    { $hidden-additional-after-letter }
    { $additional-copy-number }
    { $hidden-additional-after-title }
