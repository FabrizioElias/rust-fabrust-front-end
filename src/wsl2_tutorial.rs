use yew::prelude::*;

#[function_component(Wsl2Tutorial)]
pub fn wsl2_tutorial() -> Html {
    html! {
        <main>
            <div>
                <img class="logo" src="https://raw.githubusercontent.com/kubernetes/minikube/master/site/assets/icons/logo.svg" alt="Minikube logo" />
                <h1>{ "Tutorial on WSL2 + Minikube and other resources" }</h1>
                <br />
                <div>
                    <h2>{ "Powershell Core" }</h2>
                    <div class="normalText">
                        <p>{ "Download the most recent release of Powershell Core from " }<a target="_blank" href="https://github.com/PowerShell/PowerShell/releases">{ "GitHub" }</a>{ " and install it." }</p>
                        <p>{ "We'll be using this version for its added security and stability. I don't recommend using the default Powershell normally embedded with Windows." }</p>
                        <p>{ "I also recommend installing the "}<a target="_blank" href="https://apps.microsoft.com/detail/9n0dx20hk701">{ "Terminal app from the Windows Store" }</a>{ " and configure it to open with Powershell Core by default." }</p>
                    </div>
                </div>
                <div>
                    <h2>{ "WSL 2" }</h2>
                    <div class="normalText">
                        <p>{ "WSL stands for Windows Subsystem for Linux and allows Windows to host a fully functional Linux distribution. " }<a target="_blank" href="https://docs.microsoft.com/en-us/windows/wsl/install#install-wsl-command">{ "Install it using the provided Microsoft instrutions." }</a></p>
                        <p>{ "To install WSL2 and an Ubuntu distribution, from Powershell Core, run:" }</p>
                        <div>
                            <p class="code">{ "wsl --install -d Ubuntu-22.04" }</p>
                        </div>
                        <p>{ "You can run other distributions but Ubuntu 22.04 is the recommended one. When it runs, it'll ask for a username and a password, you can choose whatever you like." }</p>
                        <p>{ "In case you already have WSL2 installed with some distribution, it's recommended to upgrade it. From Powershell Core, run (in case of problems, run as admin):" }</p>
                        <div>
                            <p class="code">{ "wsl --update --web-download" }</p>
                        </div>
                        <p>{ "After the installation/update completes, open the terminal app with your WSL distribution. We'll update everything in the system (this will ask for your password):" }</p>
                        <div>
                            <p class="code">{ "sudo apt-get update && sudo apt-get upgrade" }</p>
                            <p class="code">{ "systemctl status" }</p>
                        </div>
                        <p>{ "By default, systemctl should be enabled on Ubuntu and you should see something like this (ctrl+C to exit):" }</p>
                        <img class="normalImage" src="static/images/01-systemctl.png" alt="WSL2 systemctl status" />
                        <p>{ "This indicates thar systemctl is enabled on your distribution and you can go to the next section of the tutorial." }</p>
                        <p>{ "In case there's an error, you'll need to enable systemctl. Run:" }</p>
                        <div>
                            <p class="code">{ "sudo nano /etc/wsl.conf" }</p>
                        </div>
                        <p>{ "Add the following:" }</p>
                        <div>
                            <p class="code">{ "[boot]" }<br/>{ "systemd=true" }</p>
                        </div>
                        <p>{ "Reboot WSL. From Powershell Core, run:" }</p>
                        <div>
                            <p class="code">{ "wsl.exe --shutdown" }</p>
                        </div>
                        <p>{ "Go back to your WSL distribution and try the systemctl status command again. If everything is all right, go to the next section." }</p>
                    </div>
                </div>
                <div>
                    <h2>{ "Brew" }</h2>
                    <div class="normalText">
                        <p>{ "Brew is a package manager that will make installing some software very easy. " }<a target="_blank" href="https://brew.sh/">{ "You can check it our here." }</a></p>
                        <p>{ "To install Brew, run and follow the instructions:" }</p>
                        <div>
                            <p class="code">{ "/bin/bash -c \"$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)\"" }</p>
                        </div>
                        <p>{ "At the end, the script will probably ask for a few commands to be run. You need to copy those commands and run them in order (they will be at the end of the instructions after installation, you only need to run those once). For me, those commands were:" }</p>
                        <div>
                            <p class="code">{ "(echo; echo 'eval \"$(/home/linuxbrew/.linuxbrew/bin/brew shellenv)\"') >> /home/camrcam/.profile" }</p>
                            <p class="code">{ "eval \"$(/home/linuxbrew/.linuxbrew/bin/brew shellenv)\"" }</p>
                            <p class="code">{ "sudo apt-get install build-essential" }</p>
                            <p class="code">{ "brew install gcc" }</p>
                        </div>
                    </div>
                </div>
                <div>
                    <h2>{ "Docker" }</h2>
                    <div class="normalText">
                        <p>{ "Docker is a container manager that you be the base for lots of apps. " }<a target="_blank" href="https://docs.docker.com/engine/install/ubuntu/">{ "You can check it our here." }</a></p>
                        <p>{ "To install Docker, you can use Brew:" }</p>
                        <div>
                            <p class="code">{ "brew install docker" }</p>
                        </div>
                        <p>{ "If anything goes wrong, you'll need to install it manually using APT:" }</p>
                        <div>
                            <p class="code">{ "sudo apt-get update" }</p>
                            <p class="code">{ "sudo apt-get install ca-certificates curl" }</p>
                            <p class="code">{ "sudo install -m 0755 -d /etc/apt/keyrings" }</p>
                            <p class="code">{ "sudo curl -fsSL https://download.docker.com/linux/ubuntu/gpg -o /etc/apt/keyrings/docker.asc" }</p>
                            <p class="code">{ "sudo chmod a+r /etc/apt/keyrings/docker.asc" }</p>
                            <p class="code">{ "echo \"deb [arch=$(dpkg --print-architecture) signed-by=/etc/apt/keyrings/docker.asc] https://download.docker.com/linux/ubuntu $(. /etc/os-release && echo \"$VERSION_CODENAME\") stable\" | sudo tee /etc/apt/sources.list.d/docker.list > /dev/null" }</p>
                            <p class="code">{ "sudo apt-get update" }</p>
                            <p class="code">{ "sudo apt-get install docker-ce docker-ce-cli containerd.io docker-buildx-plugin docker-compose-plugin" }</p>
                            <p class="code">{ "sudo usermod -aG docker $USER && newgrp docker" }</p>
                        </div>
                        <p>{ "After installation is complete, let's check if everything is ok:" }</p>
                        <div>
                            <p class="code">{ "sudo service docker start" }</p>
                            <p class="code">{ "systemctl status docker" }</p>
                        </div>
                        <p>{ "You should see something like this:" }</p>
                        <img class="normalImage" src="static/images/02-dockerstatus.png" alt="Docker systemctl status" />
                        <p>{ "In case Docker is not active and there's an error, run the following to check what it is:" }</p>
                        <div>
                            <p class="code">{ "dockerd" }</p>
                        </div>
                        <p>{ "If there's an error about IPTABLES, run, the try the command to start Docker again and check its status:" }</p>
                        <div>
                            <p class="code">{ "sudo update-alternatives --set iptables /usr/sbin/iptables-legacy && sudo update-alternatives --set ip6tables /usr/sbin/" }</p>
                        </div>
                    </div>
                </div>
                <div>
                    <h2>{ "Redis Stack" }</h2>
                    <div class="normalText">
                        <p>{ "Redis Stack is a cache solution that is fast and reliable. " }<a target="_blank" href="https://redis.io/docs/latest/operate/oss_and_stack/install/install-stack/docker/">{ "You can check it our here." }</a></p>
                        <p>{ "To install Redis Stack using Docker, run:" }</p>
                        <div>
                            <p class="code">{ "docker run -d --name redis-stack -p 6379:6379 -p 8801:8001 redis/redis-stack:latest" }</p>
                        </div>
                        <p>{ "You'll be able to access the dashboard at" }</p>
                        <div>
                            <p class="code">{ "http://localhost:8801/redis-stack/browser" }</p>
                        </div>
                        <p>{ "Every time you start WSL and want to use Redis, run:" }</p>
                        <div>
                            <p class="code">{ "docker start redis-stack" }</p>
                        </div>
                    </div>
                </div>
                <div>
                    <h2>{ "MongoDb" }</h2>
                    <div class="normalText">
                        <p>{ "MongoDb is a schemaless database that stores data as documents. " }<a target="_blank" href="https://www.mongodb.com/try/download/community-edition/releases">{ "You can check it our here." }</a></p>
                        <p>{ "To install MongoDb using Docker, run:" }</p>
                        <div>
                            <p class="code">{ "docker run --name mongodb -p 27017:27017 -d mongo:latest" }</p>
                        </div>
                        <p>{ "It's recommended to install Compass to access the database: "}<a target="_blank" href=" https://www.mongodb.com/try/download/compass">{ "You can get it here." }</a></p>
                        <p>{ "The default connection string is:" }</p>
                        <div>
                            <p class="code">{ "mongodb://localhost:27017" }</p>
                        </div>
                        <p>{ "Every time you start WSL and want to use MongoDb, run:" }</p>
                        <div>
                            <p class="code">{ "docker start mongodb" }</p>
                        </div>
                    </div>
                </div>
                <div>
                    <h2>{ "RabbitMQ" }</h2>
                    <div class="normalText">
                        <p>{ "RabbitMQ is a messaging and event manager. " }<a target="_blank" href="https://www.rabbitmq.com/docs/download">{ "You can check it our here." }</a></p>
                        <p>{ "To install RabbitMQ using Docker, run:" }</p>
                        <div>
                            <p class="code">{ "docker run -d --hostname local-rabbit --name rabbitmq -p 15672:15672 -p 5672:5672 rabbitmq:management" }</p>
                        </div>
                        <p>{ "The management dashboard can be accessed at the URL bellow with the username guest and password guest:" }</p>
                        <div>
                            <p class="code">{ "localhost:15672" }</p>
                        </div>
                        <p>{ "Every time you start WSL and want to use RabbitMQ, run:" }</p>
                        <div>
                            <p class="code">{ "sudo docker start rabbitmq" }</p>
                        </div>
                        <p>{ "In case of problems, you can check the logs by running:" }</p>
                        <div>
                            <p class="code">{ "docker logs rabbitmq" }</p>
                        </div>
                    </div>
                </div>
                <div>
                    <h2>{ "SQL Server Developer Edition" }</h2>
                    <div class="normalText">
                        <p>{ "SQL Server is a database form Microsoft. Before installation, you need to decide if you want to use the Full-Text Search feature. If not, carry on with this section. If you need FTS, go to the next section." }</p>
                        <p>{ "To install SQL Server using Docker, change the password bellow and run:" }</p>
                        <div>
                        <p class="code">{ "sudo docker run -e \"ACCEPT_EULA=Y\" -e \"SA_PASSWORD=SenhaSAa1b2c3d4!!@@\" -p 1433:1433 --name wslsql --hostname wslsql -d mcr.microsoft.com/mssql/server:2022-latest" }</p>
                        </div>
                        <p>{ "TO avoid losing data, we must persist everything even when the pod is not running by creating a volume. Run:" }</p>
                        <div>
                            <p class="code">{ "sudo docker volume create SQLServerVolume" }</p>
                        </div>
                        <p>{ "You'll need the mount point (probably something like /var/lib/docker/volumes/SQLServerVolume/_data), run:" }</p>
                        <div>
                            <p class="code">{ "sudo docker volume inspect SQLServerVolume" }</p>
                        </div>
                        <p>{ "With the mount point, we need to change its owner to the default SQL user ID. Run:" }</p>
                        <div>
                            <p class="code">{ "sudo docker exec -it wslsql bash" }</p>
                            <p class="code">{ "id" }</p>
                        </div>
                        <p>{ "The Id will be something like \"uid=10001(mssql) gid=0(root) groups=0(root)\". The id we need is the uid, in this case, 10001. Run (mind the mount point directory):" }</p>
                        <div>
                            <p class="code">{ "exit" }</p>
                            <p class="code">{ "sudo -i" }</p>
                            <p class="code">{ "cd /var/lib/docker/volumes/SQLServerVolume" }</p>
                            <p class="code">{ "ls -l" }</p>
                            <p class="code">{ "chown 10001 _data" }</p>
                        </div>
                        <p>{ "We need to reinstall SQL server. Run (remember to change the password):" }</p>
                        <div>
                            <p class="code">{ "sudo docker container rm wslsql -f" }</p>
                            <p class="code">{ "sudo docker run -e \"ACCEPT_EULA=Y\" -e \"MSSQL_SA_PASSWORD=SenhaSAa1b2c3d4!!@@\" -p  1433:1433 --name wslsql --hostname wslsql -v SQLServerVolume:/var/opt/mssql/data -d mcr.microsoft.com/mssql/server:2022-latest" }</p>
                        </div>
                        <p>{ "Check if the connection is ready. Run:" }</p>
                        <div>
                            <p class="code">{ "sudo docker exec -t wslsql cat /var/opt/mssql/log/errorlog | grep connection" }</p>
                        </div>
                        <p>{ "Every time you start WSL and want to use SQL Server, run:" }</p>
                        <div>
                            <p class="code">{ "sudo docker start wslsql" }</p>
                        </div>
                        <p>{ "To connect to the database, use the localhost IPv6 format, the username sa and the password you have set:" }</p>
                        <div>
                            <p class="code">{ "[::1]" }</p>
                        </div>
                        <p>{ "To restore backups, you need to copy the file into the SQL container. Run:" }</p>
                        <div>
                            <p class="code">{ "sudo docker exec -it wslsql /bin/bash" }</p>
                            <p class="code">{ "cd /usr" }</p>
                            <p class="code">{ "mkdir backups" }</p>
                        </div>
                        <p>{ "Exit and from the WSL terminal, run:" }</p>
                        <div>
                            <p class="code">{ "cd /mnt/c/<DIRECTORY INSIDE WINDOWS>" }</p>
                            <p class="code">{ "sudo docker cp <FILENAME>.bak wslsql:/usr/backups" }</p>
                        </div>
                        <p>{ "You can now restore the backup using SQL Manager on Windows. To get backups out of the container, run:" }</p>
                        <div>
                            <p class="code">{ "sudo docker cp wslsql:/usr/backups/<FILENAME>.bak <FILENAME>.bak" }</p>
                        </div>
                    </div>
                </div>
                <div>
                    <h2>{ "SQL Server Developer Edition with Full-Text Search" }</h2>
                    <div class="normalText">
                        <p>{ "SQL Server is a database form Microsoft. Before installation, you need to decide if you want to use the Full-Text Search feature. If yes, carry on with this section. If not, go back to the previous section." }</p>
                        <p>{ "The Dockerfile bellow is designed to work on Ubuntu 22.04 and use SQL Server 2022." }</p>
                        <p>{ "If the 2022 SQL Server version or if Ubuntu 22.04 does not work for you, go to the " }<a target="_blank" href="https://packages.microsoft.com/config/">{ "Microsoft package config repository" }</a>{ " on your browser to the find the appropriate version of Linux and SQL Server for you and replace the appropriate line bellow (read the comments)." }</p>
                        <p>{ "Use the following Dockerfile to build the image with SQL Full-Text Search enabled:" }</p>
                        <div>
                            <p class="code">
                            { "# Source: https://hub.docker.com/r/johnfarnea/mssql-server-linux-fts " }<br/>
                            { "# Modified to use the latest version of SQL Server" }<br/>
                            { "# 20220408 modified by Christof Wiegand" }<br/>
                            { "" }<br/>
                            { "# If the 2022 version does not work, go to https://hub.docker.com/_/microsoft-mssql-server and choose the correct version." }<br/>
                            { "FROM mcr.microsoft.com/mssql/server:2022-latest" }<br/>
                            { "USER root" }<br/>
                            { "" }<br/>
                            { "# Install curl since it is needed to get repo config" }<br/>
                            { "# install gnupg2 (not installed in the image)" }<br/>
                            { "# see also workaround curl with | tac | tac | : https://syntaxfix.com/question/16804/why-does-curl-return-error-23-failed-writing-body" }<br/>
                            { "" }<br/>
                            { "# Replace the URL for the appropriate config file bellow." }<br/>
                            { "RUN export DEBIAN_FRONTEND=noninteractive && apt-get update --fix-missing && apt-get install -y gnupg2 && apt-get install -yq curl apt-transport-https && curl https://packages.microsoft.com/keys/microsoft.asc | tac | tac | apt-key add - && curl https://packages.microsoft.com/config/ubuntu/22.04/mssql-server-2022.list | tac | tac | tee /etc/apt/sources.list.d/mssql-server.list && apt-get update" }<br/>
                            { "" }<br/>
                            { "# Install optional packages. Comment out the ones you don't need" }<br/>
                            { "# RUN apt-get install -y mssql-server-agent" }<br/>
                            { "# RUN apt-get install -y mssql-server-is" }<br/>
                            { "" }<br/>
                            { "RUN apt-get install -y mssql-server-fts" }<br/>
                            { "" }<br/>
                            { "# Run SQL Server process" }<br/>
                            { "CMD /opt/mssql/bin/sqlservr" }</p>
                        </div>
                        <p>{ "Save this Dockerfile somewhere easy to find in your WSL distribution. If you have SQL running on Docker already, it's recommended to remove it and any volumes associated with it:" }</p>
                        <div>
                            <p class="code">{ "sudo docker container stop wslsql" }</p>
                            <p class="code">{ "sudo docker container rm wslsql" }</p>
                            <p class="code">{ "sudo docker volume rm SQLServerVolume" }</p>
                        </div>
                        <p>{ "Recreate the volume, build the image using the previous Dockerfile and start the container (run these commands at the same folder the Dockerfile above is located), remember to change the password on the docker run command:" }</p>
                        <div>
                            <p class="code">{ "sudo docker volume create SQLServerVolume" }</p>
                            <p class="code">{ "sudo docker build -t wslsql-fts:2022-latest ." }</p>
                            <p class="code">{ "sudo docker run --name wslsql-fts --hostname wslsql-fts -e 'ACCEPT_EULA=Y' -e 'MSSQL_SA_PASSWORD=SenhaSAa1b2c3d4!!@@' -e 'MSSQL_AGENT_ENABLED=true' -p 1433:1433  -v SQLServerVolume:/var/opt/mssql/data -d wslsql-fts:2022-latest" }</p>
                        </div>
                        <p>{ "You can check the log to see if the SQL Server is ready:" }</p>
                        <div>
                            <p class="code">{ "sudo docker logs wslsql-fts" }</p>
                            <p class="code">{ "sudo docker exec -t wslsql-fts cat /var/opt/mssql/log/errorlog | grep connection" }</p>
                        </div>
                    </div>
                </div>
                <div>
                    <h2>{ "PoorMans SQL Formatter using SQL Manager 19" }</h2>
                    <div class="normalText">
                        <p>{ "After installing SQL Manager 19, install "}<a target="_blank" href=" https://simul-europe.com/PoorMansTSqlFormatterSSMSPackage.Setup.msi">{ "PoorMansTSqlFormatterSSMSPackage" }</a>{ ". Go to C:\\Program Files (x86)\\Microsoft SQL Server Management Studio 19\\Common7\\IDE and edit the file ssms.exe.config. In the tag <assemblyBinding> Add:" }</p>
                        <div>
                            <p class="code">{ "<dependentAssembly> <assemblyIdentity name=\"Microsoft.VisualStudio.Shell.12.0\" publicKeyToken=\"b03f5f7f11d50a3a\" culture=\"neutral\"/> <bindingRedirect oldVersion=\"2.0.0.0-15.0.0.0\" newVersion=\"15.0.0.0\"/> </dependentAssembly>" }</p>
                        </div>
                    </div>
                </div>
                <div>
                    <h2>{ "kubectl" }</h2>
                    <div class="normalText">
                        <p>{ "By default, Minikube uses an internal kubectl tool, but using it along with K9s leaves it in a limited mode. To use a real kubectl, run:" }</p>
                        <div>
                            <p class="code">{ "sudo apt-get install -y apt-transport-https ca-certificates curl" }</p>
                            <p class="code">{ "echo \"deb [signed-by=/etc/apt/keyrings/kubernetes-apt-keyring.gpg] https://pkgs.k8s.io/core:/stable:/v1.28/deb/ /\" | sudo tee /etc/apt/sources.list.d/kubernetes.list" }</p>
                            <p class="code">{ "curl -fsSL https://pkgs.k8s.io/core:/stable:/v1.28/deb/Release.key | sudo gpg --dearmor -o /etc/apt/keyrings/kubernetes-apt-keyring.gpg" }</p>
                            <p class="code">{ "sudo apt-get update" }</p>
                            <p class="code">{ "sudo apt-get install -y kubectl" }</p>
                        </div>
                        <p>{ "During Minikube installation, it detects the external kubectl and uses it." }</p>
                    </div>
                </div>
                <div>
                    <h2>{ "Minikube" }</h2>
                    <div class="normalText">
                        <p>{ "To install Minikube, run:" }</p>
                        <div>
                            <p class="code">{ "curl -LO https://storage.googleapis.com/minikube/releases/latest/minikube-linux-amd64" }</p>
                            <p class="code">{ "sudo install minikube-linux-amd64 /usr/local/bin/minikube" }</p>
                        </div>
                        <p>{ "To update Minikube, first stop it (or delete it) and re-run the above commands." }</p>
                        <div>
                            <p class="code">{ "minikube stop" }</p>
                            <p class="code">{ "minikube delete" }</p>
                        </div>
                    </div>
                </div>
                <div>
                    <h2>{ "K9s" }</h2>
                    <div class="normalText">
                        <p>{ "To install K9s (Kubernetes GUI manager), run:" }</p>
                        <div>
                            <p class="code">{ "brew install k9s" }</p>
                        </div>
                        <p>{ "By default K9s uses Vim to edit deployments, if you prefer to user Nano, run the following command before starting K9s:" }</p>
                        <div>
                            <p class="code">{ "export EDITOR=nano" }</p>
                        </div>
                    </div>
                </div>
                <div>
                    <h2>{ "Starting Minikube" }</h2>
                    <div class="normalText">
                        <p>{ "To start Minikube configured to user Docker, por 5000 and a local Docker repository, run:" }</p>
                        <div>
                            <p class="code">{ "minikube start --driver=docker --container-runtime=docker --ports=5000:5000 --insecure-registry \"10.0.0.0/24\" --extra-config=kubelet.housekeeping-interval=1s && eval $(minikube -p minikube docker-env)" }</p>
                        </div>
                        <p>{ "After it starts for the first time, we need to configure some extensions (Load balancer, metrics and ingress). This needs to be done only the first time Minikube is started:" }</p>
                        <div>
                            <p class="code">{ "minikube addons enable metallb" }</p>
                            <p class="code">{ "minikube addons enable metrics-server" }</p>
                            <p class="code">{ "minikube addons enable ingress " }</p>
                            <p class="code">{ "minikube addons enable ingress-dns" }</p>
                        </div>
                        <p>{ "To configure the Load balancer, we need to find the internal IP used by Minikube:" }</p>
                        <div>
                            <p class="code">{ "minikube ssh" }</p>
                            <p class="code">{ "ping host.minikube.internal" }</p>
                        </div>
                        <p>{ "Let it run for a few seconds, stop it using ctrl+C and not the IP address (should be 192.168.49.1). Exit the SSH session and configure MetalLB:" }</p>
                        <div>
                            <p class="code">{ "exit" }</p>
                            <p class="code">{ "minikube addons configure metallb" }</p>
                        </div>
                        <p>{ "MetalLB will ask for a start and end IP address. It will use this range to assign to services. It's recommended this range has the most possible IPs addresses available inside the Minikube domain (previous command), for example:" }</p>
                        <div>
                            <p class="code">{ "192.168.49.100" }</p>
                            <p class="code">{ "192.168.49.150" }</p>
                        </div>
                        <p>{ "To check if Minikube is configured correctly, you can run:" }</p>
                        <div>
                            <p class="code">{ "docker ps -a" }</p>
                        </div>
                        <p>{ "If you see the Minikube containers, it means you're using the external Docker instance. You need to use the internal one. To enable it in the current terminal session, run:" }</p>
                        <div>
                            <p class="code">{ "eval $(minikube -p minikube docker-env)" }</p>
                        </div>
                        <p>{ "To enable the local Docker registry, run:" }</p>
                        <div>
                            <p class="code">{ "minikube ssh" }</p>
                            <p class="code">{ "docker run --restart=always -d -p 5000:5000 --name registry registry:2" }</p>
                        </div>
                        <p>{ "To test it, run:" }</p>
                        <div>
                            <p class="code">{ "curl 127.0.0.1:5000/v2/_catalog" }</p>
                        </div>
                        <p>{ "For an empty repository, the response should be:" }</p>
                        <div>
                            <p class="code">{ "{\"repositories\":[]}" }</p>
                        </div>
                        <p>{ "Now you can user docker push normally (more details about this process can be " }<a target="_blank" href=" https://stackoverflow.com/a/65589218">{ "found here" }</a>{")" }</p>
                    </div>
                </div>
                <div>
                    <h2>{ "Run something inside Minikube" }</h2>
                    <div class="normalText">
                        <p>{ "Navigate to your repo folder. You can find the Windows folders at /mnt/c" }</p>
                        <p>{ "Navigate to the folder with your Dockerfile. It's not recommended to user the latest tag (or no tag at all), since this can make Docker search the image on it's online repository, and not the local one. I like to use the tag minikube  (replace filenames):" }</p>
                        <div>
                            <p class="code">{ "docker build . -t <IMAGE NAME>:minikube -f <DOCKERFILE NAME>" }</p>
                        </div>
                        <p>{ "For example:" }</p>
                        <div>
                            <p class="code">{ "docker build . -t myfirstproject:minikube -f dev-docker" }</p>
                        </div>
                        <p>{ "To build an image, push it into the internal Docker repository and apply its Kubernetes configuration/deploy (replace filenames), run:" }</p>
                        <div>
                            <p class="code">{ "docker build . -t localhost:5000/<IMAGE NAME>:minikube -f <DOCKERFILE NAME> && docker push localhost:5000/<IMAGE NAME>:minikube && kubectl apply -f <DEPLOY FILENAME>.yaml" }</p>
                        </div>
                        <p>{ "Remember to put the same image name in your deploys Yaml file, including the tag. Images that should be downloaded from the Docker Hub should have the normal image name and latest tag." }</p>
                        <p>{ "If you have any connection strings on your configurations, replace them for the IP os the service, or if the service is running outside of Minikube (i.e. on the host), you can use \"host.minikube.internal\" as the address." }</p>
                        <p>{ "Execute K9s by running:" }</p>
                        <div>
                            <p class="code">{ "k9s" }</p>
                        </div>
                        <p>{ "In the unusual cae K9s cannot find your Minikube instance (configuration error), you can check if the configurations are indeed present by running:" }</p>
                        <div>
                            <p class="code">{ "kubectl config view --kubeconfig $HOME/.kube/config" }</p>
                        </div>
                        <p>{ "If not, you can create the folders .kube/config using the mkdir command and running the above command again. If the configurations are still not present, restart Minikube by running the stop and the start command seen previously." }</p>
                        <p>{ "If the configurations are still not preset, reboot WSL by running the following commando in Powershell Core in Admin mode, the restarting Minikube and K9s:" }</p>
                        <div>
                            <p class="code">{ "wsl --shutdown" }</p>
                        </div>
                        <p>{ "K9s should open in the default namespace. Get yourself used with the interface using the " }<a target="_blank" href="https://k9scli.io/">{ "documentation found here" }</a>{". If you wish to keep running commands against Minikube, open another terminal session/another tab (remember to run the eval command). To exit K9s, press ctrl+C." }</p>
                        <p>{ "Pay special attention if the services have assigned IP addresses. If you wish to access services running inside Minikube from the outside, you should open a new terminal session and run:" }</p>
                        <div>
                            <p class="code">{ "minikube tunnel" }</p>
                        </div>
                    </div>
                </div>
                <div>
                    <h2>{ "Load testing with K6" }</h2>
                    <div class="normalText">
                        <p>{ "To install K6 (Load test tool) using Brew, run:" }</p>
                        <div>
                            <p class="code">{ "brew install k6" }</p>
                        </div>
                        <p>{ "You should first familiarize yourself with the K6 scripts by reading" }<a target="_blank" href="https://k6.io/docs/getting-started/running-k6/ ">{ "the documentation" }</a>{". To run scripts, run (replace filename):" }</p>
                        <div>
                            <p class="code">{ "k6 run <SCRIPT NAME>.js" }</p>
                        </div>
                        <p>{ "It's recommended to create a folder with your scripts inside. If you wish to record test execution parameters or see in realtime the test running is a visual manner, you can install InfluxDB and Grafana:" }</p>
                        <div>
                            <p class="code">{ "brew install influxdb" }</p>
                            <p class="code">{ "brew install grafana" }</p>
                        </div>
                        <p>{ "The Grafana GUI can be accessed on \"localhost:3000\" by default. It's recommended to add the" }<a target="_blank" href="https://grafana.com/grafana/dashboards/2587 ">{ "2587 dashboard" }</a>{ ". To run K6 and store the result, run:" }</p>
                        <div>
                            <p class="code">{ "k6 run --out influxdb=http://localhost:8086/myk6db <SCRIPT NAME>.js" }</p>
                        </div>
                        <p>{ "Running this command will instruct K6 to store the test results on the myk6db database in InfluxDB. These results can be seen in the Grafana dashboard. If you want to delete old results, run:" }</p>
                        <div>
                            <p class="code">{ "sudo apt-get install influx-client" }</p>
                            <p class="code">{ "Influxdb" }</p>
                            <p class="code">{ "use myk6db" }</p>
                            <p class="code">{ "delete from myk6db where time < now()" }</p>
                        </div>
                        <p>{ "In case Grafana stops, run:" }</p>
                        <div>
                            <p class="code">{ "sudo systemctl restart grafana-server" }</p>
                        </div>
                    </div>
                </div>
                <div>
                    <p>{ "To run back end, execute in terminal the following command from inside /back-end:" }</p>
                    <p>{ "cargo run" }</p>
                    <p>{ "If you want live reloading, run:" }</p>
                    <p>{ "cargo watch -c -q -x run" }</p>
                    <p>{ "backend runs on localhost:3000" }</p>
                    <p>{ "To run front end, execute in terminal the following command to enable live reloading from inside /front-end:" }</p>
                    <p>{ "Install WASM target" }</p>
                    <p>{ "rustup target add wasm32-unknown-unknown" }</p>
                    <p>{ "Install trunk" }</p>
                    <p>{ "cargo install --locked trunk" }</p>
                    <p>{ "Serve the app" }</p>
                    <p>{ "trunk serve" }</p>
                    <p>{ "Serve the app and open the browser" }</p>
                    <p>{ "trunk serve --open" }</p>
                    <p>{ "frontend runs on localhost:8080 by default or whats configured on Trunk.toml" }</p>
                </div>
            </div>
        </main>
    }
}