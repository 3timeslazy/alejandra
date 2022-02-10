pub fn rule(
    build_ctx: &crate::builder::BuildCtx,
    node: &rnix::SyntaxNode,
) -> std::collections::LinkedList<crate::builder::Step> {
    let mut steps = std::collections::LinkedList::new();

    let mut children = crate::children::Children::new_with_configuration(
        build_ctx, node, true,
    );

    let layout = if children.has_comments() || children.has_newlines() {
        &crate::config::Layout::Tall
    } else {
        build_ctx.config.layout()
    };

    // a
    let child = children.get_next().unwrap();
    match layout {
        crate::config::Layout::Tall => {
            steps.push_back(crate::builder::Step::FormatWider(child.element));
        }
        crate::config::Layout::Wide => {
            steps.push_back(crate::builder::Step::Format(child.element));
        }
    }

    // /**/
    children.drain_comments_and_newlines(|element| match element {
        crate::children::DrainCommentOrNewline::Comment(text) => {
            steps.push_back(crate::builder::Step::NewLine);
            steps.push_back(crate::builder::Step::Pad);
            steps.push_back(crate::builder::Step::Comment(text));
        }
        crate::children::DrainCommentOrNewline::Newline(_) => {}
    });

    if let rnix::SyntaxKind::TOKEN_COMMENT
    | rnix::SyntaxKind::TOKEN_WHITESPACE =
        children.peek_prev().unwrap().element.kind()
    {
        steps.push_back(crate::builder::Step::NewLine);
        steps.push_back(crate::builder::Step::Pad);
    } else {
        steps.push_back(crate::builder::Step::Whitespace);
    }

    // =
    let mut dedent = false;
    let child = children.get_next().unwrap();
    steps.push_back(crate::builder::Step::Format(child.element));
    match layout {
        crate::config::Layout::Tall => {
            let next = children.peek_next().unwrap();
            let next_kind = next.element.kind();

            if let rnix::SyntaxKind::NODE_ATTR_SET
            | rnix::SyntaxKind::NODE_LIST
            | rnix::SyntaxKind::NODE_PAREN
            | rnix::SyntaxKind::NODE_STRING = next_kind
            {
                steps.push_back(crate::builder::Step::Whitespace);
            } else if let rnix::SyntaxKind::NODE_APPLY = next_kind {
                if let rnix::SyntaxKind::NODE_ATTR_SET
                | rnix::SyntaxKind::NODE_LIST
                | rnix::SyntaxKind::NODE_PAREN
                | rnix::SyntaxKind::NODE_STRING = next
                    .element
                    .into_node()
                    .unwrap()
                    .children()
                    .collect::<Vec<rnix::SyntaxNode>>()
                    .iter()
                    .rev()
                    .next()
                    .unwrap()
                    .kind()
                {
                    steps.push_back(crate::builder::Step::Whitespace);
                } else {
                    dedent = true;
                    steps.push_back(crate::builder::Step::Indent);
                    steps.push_back(crate::builder::Step::NewLine);
                    steps.push_back(crate::builder::Step::Pad);
                }
            } else if let rnix::SyntaxKind::NODE_LAMBDA = next_kind {
                if let rnix::SyntaxKind::NODE_PATTERN = next
                    .element
                    .into_node()
                    .unwrap()
                    .children()
                    .next()
                    .unwrap()
                    .kind()
                {
                    dedent = true;
                    steps.push_back(crate::builder::Step::Indent);
                    steps.push_back(crate::builder::Step::NewLine);
                    steps.push_back(crate::builder::Step::Pad);
                } else {
                    steps.push_back(crate::builder::Step::Whitespace);
                }
            } else {
                dedent = true;
                steps.push_back(crate::builder::Step::Indent);
                steps.push_back(crate::builder::Step::NewLine);
                steps.push_back(crate::builder::Step::Pad);
            }
        }
        crate::config::Layout::Wide => {
            steps.push_back(crate::builder::Step::Whitespace);
        }
    }

    // /**/
    children.drain_comments_and_newlines(|element| match element {
        crate::children::DrainCommentOrNewline::Comment(text) => {
            steps.push_back(crate::builder::Step::Comment(text));
            steps.push_back(crate::builder::Step::NewLine);
            steps.push_back(crate::builder::Step::Pad);
        }
        crate::children::DrainCommentOrNewline::Newline(_) => {}
    });

    // b
    let child = children.get_next().unwrap();
    match layout {
        crate::config::Layout::Tall => {
            steps.push_back(crate::builder::Step::FormatWider(child.element));
        }
        crate::config::Layout::Wide => {
            steps.push_back(crate::builder::Step::Format(child.element));
        }
    }

    // /**/
    children.drain_comments_and_newlines(|element| match element {
        crate::children::DrainCommentOrNewline::Comment(text) => {
            steps.push_back(crate::builder::Step::NewLine);
            steps.push_back(crate::builder::Step::Pad);
            steps.push_back(crate::builder::Step::Comment(text));
        }
        crate::children::DrainCommentOrNewline::Newline(_) => {}
    });

    if let rnix::SyntaxKind::TOKEN_COMMENT
    | rnix::SyntaxKind::TOKEN_WHITESPACE =
        children.peek_prev().unwrap().element.kind()
    {
        steps.push_back(crate::builder::Step::NewLine);
        steps.push_back(crate::builder::Step::Pad);
    }

    // ;
    let child = children.get_next().unwrap();
    steps.push_back(crate::builder::Step::Format(child.element));
    if dedent {
        steps.push_back(crate::builder::Step::Dedent);
    }

    steps
}
