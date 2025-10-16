import React from "react";
import { Card, CardContent, CardFooter, CardHeader, CardTitle, CardDescription, CardAction } from "@/components/ui/card"
import { Button } from "@/components/ui/button"
import "./App.css";
import { invoke } from "@tauri-apps/api/core";
import { ExternalLink } from "lucide-react";

type AppType = {
    name: string;
    url: string;
    icon?: string;
}

function App() {
    const [apps, setApps] = React.useState<AppType[]>([]);

    React.useEffect(() => {
        invoke("get_app_data").then((data) => {
            console.log(data);
            setApps(data as AppType[]);
        }).catch((err) => {
            console.error(err);
        });
    }, [])

    const handleClick = (app: AppType) => {
        invoke("run_app", { appName: app.name }).catch((err) => {
            console.error(err);
        });
    }

  return (
    <main className="container">
      <h1>Welcome to Tauri + React</h1>
      <ul>
        {apps.map((app, index) => (
            <li key={index}>
                <Card>
                  <CardHeader>
                    <CardTitle>{app.name}</CardTitle>
                    <CardDescription>{app.url}</CardDescription>
                    <CardAction>
                      <Button onClick={() => handleClick(app)}><ExternalLink /></Button>
                    </CardAction>
                  </CardHeader>
                  <CardContent>
                    <p>Card Content</p>
                  </CardContent>
                  <CardFooter>
                    <p>Card Footer</p>
                  </CardFooter>
                </Card>
                {app.icon && <img src={app.icon} alt={`${app.name} icon`} width={32} height={32} />}
                <a href={app.url} target="_blank" rel="noopener noreferrer">{app.name}</a>
            </li>
        ))}
    </ul>
    </main>
  );
}

export default App;
