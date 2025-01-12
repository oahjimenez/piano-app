##########################################
# -------------- Rust --------------------
##########################################

#Creation projet
https://code.visualstudio.com/docs/languages/rust

#Rust analyser -> autodiscovery works at most 1 subfolder deep, auto discovery will fail otherwise
https://users.rust-lang.org/t/in-vscode-rust-analyzer-features-such-as-code-completion-doesnt-work-in-certain-conditions/107685/5


##########################################
# -------------- React --------------------
##########################################
#Installation
https://dev.to/code_jedi/how-to-install-reactjs-on-macos-4hio

#Error Could not resolve dependency: npm error peer react@"^18.0.0
Supprimer node_modules, supprimer package-json.lock, décrémenter la version react, npm install

#Erreur compilation web-vitals manquant	
npm i web-vitals --save-dev

#MongoDB Rust configuration
https://www.mongodb.com/developer/languages/rust/rust-mongodb-crud-tutorial/

#MongoDB Cluster
https://cloud.mongodb.com/v2/677c07caa60ba83a335dd142#/overview?automateSecurity=true&connectCluster=RustQuickstart

Database user
username: oswaldojimenezhidalgo
password: flO9qhklhlTd7u8B

#Proxy cors error
https://dev.to/codeofrelevancy/how-to-set-up-a-proxy-server-in-react-dealing-with-cors-87e
Solution (pas valide pour un environnement non productif: ajouter actix-cors -> autoriser le host qui appelle coté backend rust)

#react scss integration - craco
https://stackoverflow.com/questions/67777943/how-to-setup-webpack-using-create-react-app-for-sass-resources-loader
https://stackoverflow.com/questions/66820999/react-cannot-start-project-craco-is-not-recognized-as-an-internal-or-externa
npm install sass-loader sass webpack --save-dev
