use gtk::prelude::*;
use webkit6::prelude::*;
use webkit6::{NavigationPolicyDecision, PolicyDecisionType, WebView};

// Prevents the WebView from navigating to external links
pub fn setup_navigation(previewer: &WebView) {
    previewer.connect_decide_policy(|_webview, decision, decision_type| {
        if decision_type == PolicyDecisionType::NavigationAction {
            if let Ok(nav_decision) = decision.clone().downcast::<NavigationPolicyDecision>() {
                if let Some(mut nav_action) = nav_decision.navigation_action() {
                    if let Some(request) = nav_action.request() {
                        if let Some(uri) = request.uri() {
                            if !uri.starts_with("about:blank") {
                                eprintln!("Blocked external link: {}", uri);
                                nav_decision.ignore();
                                return true;
                            }
                        }
                    }
                }
            }
        }
        false
    });
}
