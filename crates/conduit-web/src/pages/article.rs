use yew::prelude::*;

use crate::{
    contexts::authentication_context::use_authentication_context,
    hooks::use_selected_article::{use_selected_article, UseSelectedArticleHook},
};

#[derive(Properties, PartialEq, Clone)]
pub struct ArticleProps {
    pub slug: String,
}

#[function_component(Article)]
pub fn article(props: &ArticleProps) -> Html {
    let UseSelectedArticleHook { article } = use_selected_article(props.slug.clone());
    let authentication_context = use_authentication_context();

    let maybe_follow_and_post_buttons = {
        let authentication_context = authentication_context.clone();

        move || -> Html {
            if authentication_context.is_authenticated() {
                html! {
                    <>
                        <button class="btn btn-sm btn-outline-secondary">
                            <i class="ion-plus-round"></i>
                            {"\u{00a0}Follow Eric Simons"} <span class="counter">{" \u{0028}10\u{0029}"}</span>
                        </button>
                        {"\u{00a0}\u{00a0}"}
                        <button class="btn btn-sm btn-outline-primary">
                            <i class="ion-heart"></i>
                            {"\u{00a0}Favorite Post"} <span class="counter">{" \u{0028}29\u{0029}"}</span>
                        </button>
                    </>
                }
            } else {
                html! {}
            }
        }
    };

    let maybe_comment_box = {
        let authentication_context = authentication_context.clone();

        move || -> Html {
            if authentication_context.is_authenticated() {
                html! {
                    <form class="card comment-form">
                        <div class="card-block">
                            <textarea class="form-control" placeholder="Write a comment..." rows="3"></textarea>
                        </div>
                        <div class="card-footer">
                            <img src="http://i.imgur.com/Qr71crq.jpg" class="comment-author-img" />
                            <button class="btn btn-sm btn-primary">
                                {"Post Comment"}
                            </button>
                        </div>
                    </form>
                }
            } else {
                html! {}
            }
        }
    };

    let maybe_edit_comment_options = {
        move || -> Html {
            if authentication_context.is_authenticated() {
                html! {
                    <span class="mod-options">
                        <i class="ion-edit"></i>
                        <i class="ion-trash-a"></i>
                    </span>
                }
            } else {
                html! {}
            }
        }
    };

    html! {
        <div class="article-page">
            <div class="banner">
                <div class="container">
                    <h1>{article.title.clone()}</h1>

                    <div class="article-meta">
                        <a href=""><img src="http://i.imgur.com/Qr71crq.jpg" /></a>
                        <div class="info">
                            <a href="" class="author">{"Eric Simons"}</a>
                            <span class="date">{"January 20th"}</span>
                        </div>
                        {maybe_follow_and_post_buttons()}
                    </div>
                </div>
            </div>

            <div class="container page">
                <div class="row article-content">
                    <div class="col-md-12">
                        <p>
                            {article.description}
                        </p>
                        <h2 id="introducing-ionic">{article.title}</h2>
                        <p>{article.body}</p>
                    </div>
                </div>

                <hr />

                <div class="article-actions">
                    <div class="article-meta">
                        <a href="profile.html"><img src="http://i.imgur.com/Qr71crq.jpg" /></a>
                        <div class="info">
                            <a href="" class="author">{"Eric Simons"}</a>
                            <span class="date">{"January 20th"}</span>
                        </div>
                        {maybe_follow_and_post_buttons()}
                    </div>
                </div>

                <div class="row">
                    <div class="col-xs-12 col-md-8 offset-md-2">
                        {maybe_comment_box()}

                        <div class="card">
                            <div class="card-block">
                                <p class="card-text">{"With supporting text below as a natural lead-in to additional content."}</p>
                            </div>
                            <div class="card-footer">
                                <a href="" class="comment-author">
                                    <img src="http://i.imgur.com/Qr71crq.jpg" class="comment-author-img" />
                                </a>
                                {"\u{00a0}"}
                                <a href="" class="comment-author">{"Jacob Schmidt"}</a>
                                <span class="date-posted">{"Dec 29th"}</span>
                            </div>
                        </div>

                        <div class="card">
                            <div class="card-block">
                                <p class="card-text">{"With supporting text below as a natural lead-in to additional content."}</p>
                            </div>
                            <div class="card-footer">
                                <a href="" class="comment-author">
                                    <img src="http://i.imgur.com/Qr71crq.jpg" class="comment-author-img" />
                                </a>
                                {"\u{00a0}"}
                                <a href="" class="comment-author">{"Jacob Schmidt"}</a>
                                <span class="date-posted">{"Dec 29th"}</span>
                                {maybe_edit_comment_options()}
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}
