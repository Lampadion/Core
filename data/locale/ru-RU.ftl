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
    { $tag-script-at-start }
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
    { $tag-script-at-end }


# Hidden tag

hidden-tag =
    { $tag-script-at-start-hidden }
    { $type-tag-hidden }
    { $type-tag }
    { $hidden-tag-script-additive }
    { $before-title }
    { $case }
    { $before-type }
    { $type-non-germanic }
    { $hidden-after-type }
    { $hidden-before-letter }
    { $hidden-letter }
    { $hidden-after-letter }
    { $second-name }
    { $copy-number }
    { $hidden-after-title }
    { $postfix }
    { $tag-script-at-end-hidden }


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
    { $tag-script-at-start }
    { $additional-before-type }
    { $case }
    { $type-tag }
    { $additional-after-type }
    { $additional-before-letter }
    { $additional-letter }
    { $additional-after-letter }
    { $additional-copy-number }
    { $additional-after-title }
    { $tag-script-at-end }


# Additive hidden tag

additive-hidden-tag =
    { $hidden-additional-before-title }
    { $hidden-tag-script-at-start }
    { $hidden-additional-before-type }
    { $hidden-case } { $hidden-type-tag }
    { $hidden-additional-after-type }
    { $hidden-additional-before-letter }
    { $additional-hidden-letter }
    { $hidden-additional-after-letter }
    { $additional-copy-number }
    { $hidden-additional-after-title }
    { $hidden-tag-script-at-end }
