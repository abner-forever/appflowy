// DO NOT EDIT. This is code generated via package:easy_localization/generate.dart

// ignore_for_file: prefer_single_quotes

import 'dart:ui';

import 'package:easy_localization/easy_localization.dart' show AssetLoader;

class CodegenLoader extends AssetLoader{
  const CodegenLoader();

  @override
  Future<Map<String, dynamic>> load(String fullPath, Locale locale ) {
    return Future.value(mapLocales[locale.toString()]);
  }

  static const Map<String,dynamic> en = {
  "appName": "Appflowy",
  "defaultUsername": "Me",
  "welcomeText": "Welcome to @:appName",
  "githubStarText": "Star on GitHub",
  "subscribeNewsletterText": "Subscribe to Newsletter",
  "letsGoButtonText": "Let's Go",
  "title": "Title",
  "signUp": {
    "buttonText": "Sign Up",
    "title": "Sign Up to @:appName",
    "getStartedText": "Get Started",
    "emptyPasswordError": "Password can't be empty",
    "repeatPasswordEmptyError": "Repeat password can't be empty",
    "unmatchedPasswordError": "Repeat password is not the same as password",
    "alreadyHaveAnAccount": "Already have an account?",
    "emailHint": "Email",
    "passwordHint": "Password",
    "repeatPasswordHint": "Repeat password"
  },
  "signIn": {
    "loginTitle": "Login to @:appName",
    "loginButtonText": "Login",
    "buttonText": "Sign In",
    "forgotPassword": "Forgot Password?",
    "emailHint": "Email",
    "passwordHint": "Password",
    "dontHaveAnAccount": "Don't have an account?",
    "repeatPasswordEmptyError": "Repeat password can't be empty",
    "unmatchedPasswordError": "Repeat password is not the same as password"
  },
  "workspace": {
    "create": "Create workspace",
    "hint": "workspace",
    "notFoundError": "Workspace not found"
  },
  "shareAction": {
    "buttonText": "Share",
    "workInProgress": "Work in progress",
    "markdown": "Markdown",
    "copyLink": "Copy Link"
  },
  "disclosureAction": {
    "rename": "Rename",
    "delete": "Delete",
    "duplicate": "Duplicate"
  },
  "blankPageTitle": "Blank page",
  "newPageText": "New page",
  "trash": {
    "text": "Trash",
    "restoreAll": "Restore All",
    "deleteAll": "Delete All",
    "pageHeader": {
      "fileName": "File name",
      "lastModified": "Last Modified",
      "created": "Created"
    }
  },
  "deletePagePrompt": {
    "text": "This page is in Trash",
    "restore": "Restore page",
    "deletePermanent": "Delete permanently"
  },
  "dialogCreatePageNameHint": "Page name",
  "questionBubble": {
    "whatsNew": "What's new?",
    "help": "Help & Support"
  },
  "menuAppHeader": {
    "addPageTooltip": "Quickly add a page inside",
    "defaultNewPageName": "Untitles",
    "renameDialog": "Rename"
  },
  "toolbar": {
    "undo": "Undo",
    "redo": "Redo",
    "bold": "Bold",
    "italic": "Italic",
    "underline": "Underline",
    "strike": "Strikethrough",
    "numList": "Numbered List",
    "bulletList": "Bulleted List",
    "checkList": "Check List",
    "inlineCode": "Inline Code",
    "quote": "Quote Block"
  },
  "contactsPage": {
    "title": "Contacts",
    "whatsHappening": "What's happening this week?",
    "addContact": "Add Contact",
    "editContact": "Edit Contact"
  },
  "button": {
    "OK": "OK",
    "Cancel": "Cancel",
    "signIn": "Sign In",
    "signOut": "Sign Out",
    "complete": "Complete",
    "save": "Save"
  },
  "label": {
    "welcome": "Welcome!",
    "firstName": "First Name",
    "middleName": "Middle Name",
    "lastName": "Last Name",
    "stepX": "Step {X}"
  },
  "oAuth": {
    "err": {
      "failedTitle": "Unable to connect to your account.",
      "failedMsg": "Please make sure you've completed the sign-in process in your browser."
    },
    "google": {
      "title": "GOOGLE SIGN-IN",
      "instruction1": "In order to import your Google Contacts, you'll need to authorize this application using your web browser.",
      "instruction2": "Copy this code to your clipboard by clicking the icon or selecting the text:",
      "instruction3": "Navigate to the following link in your web browser, and enter the above code:",
      "instruction4": "Press the button below when you've completed signup:"
    }
  }
};
static const Map<String, Map<String,dynamic>> mapLocales = {"en": en};
}
