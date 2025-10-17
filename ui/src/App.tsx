import React from "react";
import { Card, CardContent, CardHeader, CardTitle, CardDescription } from "@/components/ui/card"
import { Button } from "@/components/ui/button"
import { Dialog, DialogContent, DialogDescription, DialogFooter, DialogHeader, DialogTitle, DialogTrigger } from "@/components/ui/dialog"
import { Input } from "@/components/ui/input"
import { Label } from "@/components/ui/label"
import "./App.css";
import { invoke } from "@tauri-apps/api/core";
import { ExternalLink, Plus, Trash2 } from "lucide-react";
import { convertFileSrc } from "@tauri-apps/api/core";

type AppType = {
    name: string;
    url: string;
    icon?: string;
}

function App() {
    const [apps, setApps] = React.useState<AppType[]>([]);
    const [imageSrcs, setImageSrc] = React.useState<{[key: string]: string}>({})
    const [createOpen, setCreateOpen] = React.useState(false);
    const [editOpen, setEditOpen] = React.useState(false);
    const [deleteOpen, setDeleteOpen] = React.useState(false);
    const [selectedApp, setSelectedApp] = React.useState<AppType | null>(null);
    const [formData, setFormData] = React.useState({ name: "", url: "", icon: "" });
 
    React.useEffect(() => {
        console.log('formDta changed:', formData);
    }, [formData])

    React.useEffect(() => {
        invoke("get_app_data").then((data) => {
            console.log(data);
            setApps(data as AppType[]);
        }).catch((err) => {
            console.error(err);
        });
    }, [])

    React.useEffect(() => {
        const sources = apps.reduce((prev, app) => {
            if (!app.icon) return prev;
            const src = convertFileSrc(app.icon);
            return { ...prev, [app.name]: src };
        }, {})

        setImageSrc(sources);
    }, [apps])

    const handleClick = (app: AppType) => {
        invoke("run_app", { appName: app.name }).catch((err) => {
            console.error(err);
        });
    }

    const handleCreateOpen = () => {
        setFormData({ name: "", url: "", icon: "" });
        setCreateOpen(true);
    };

    const handleEditOpen = (app: AppType) => {
        setSelectedApp(app);
        setEditOpen(true);
        setFormData({ name: app.name, url: app.url, icon: app.icon || "" });
    };

    const handleDeleteOpen = (app: AppType) => {
        setSelectedApp(app);
        setDeleteOpen(true);
    };

    const handleCreate = () => {
        console.log("Create app:", formData);
        setCreateOpen(false);
    };

    const handleEdit = () => {
        console.log("Edit app:", formData);
        setEditOpen(false);
    };

    const handleDelete = () => {
        console.log("Delete app:", selectedApp?.name);
        setDeleteOpen(false);
    };

  return (
      <div className="py-3 px-4 flex flex-col h-screen">
        <div className="flex justify-between items-center flex-grow max-w-7xl mx-auto w-full">
          <h1 className="text-4xl font-bold">Tarantula</h1>
          <Dialog open={createOpen} onOpenChange={setCreateOpen}>
            <DialogTrigger asChild>
              <Button onClick={handleCreateOpen} size="lg">
                <Plus className="mr-2 h-5 w-5" />
                Add App
              </Button>
            </DialogTrigger>
            <DialogContent>
              <DialogHeader>
                <DialogTitle>Create New App</DialogTitle>
                <DialogDescription>
                  Add a new application to your collection.
                </DialogDescription>
              </DialogHeader>
              <div className="grid gap-4 py-4">
                <div className="grid gap-2">
                  <Label htmlFor="create-name">Name</Label>
                  <Input
                    id="create-name"
                    value={formData.name}
                    onChange={(e) => setFormData({ ...formData, name: e.target.value })}
                    placeholder="App name"
                  />
                </div>
                <div className="grid gap-2">
                  <Label htmlFor="create-url">URL</Label>
                  <Input
                    id="create-url"
                    value={formData.url}
                    onChange={(e) => setFormData({ ...formData, url: e.target.value })}
                    placeholder="https://example.com"
                  />
                </div>
              </div>
              <DialogFooter>
                <Button variant="outline" onClick={() => setCreateOpen(false)}>
                  Cancel
                </Button>
                <Button onClick={handleCreate}>Create</Button>
              </DialogFooter>
            </DialogContent>
          </Dialog>
        </div>

        <div className="flex-grow overflow-y-auto pt-3">
            <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6 max-w-7xl mx-auto">
              {apps.map((app, index) => (
                <Card key={index} className="hover:shadow-lg transition-shadow" onClick={() => handleEditOpen(app)}>
                  <CardHeader>
                    <div className="flex items-start gap-4">
                      {app.icon && (
                        <img 
                          src={imageSrcs[app.name]} 
                          alt={`${app.name} icon`} 
                          className="w-12 h-12 rounded-lg object-cover"
                        />
                      )}
                      <div className="flex-1">
                        <CardTitle className="mb-2">{app.name}</CardTitle>
                        <CardDescription className="break-all">{app.url}</CardDescription>
                      </div>
                    </div>
                  </CardHeader>
                  <CardContent>
                    <div className="flex gap-2">
                      <Button 
                        onClick={() => handleClick(app)} 
                        className="flex-1"
                        size="sm"
                      >
                        <ExternalLink className="mr-2 h-4 w-4" />
                        Launch
                      </Button>
                      <Button 
                        onClick={() => handleDeleteOpen(app)} 
                        variant="destructive"
                        size="sm"
                      >
                        <Trash2 className="h-4 w-4" />
                      </Button>
                    </div>
                  </CardContent>
                </Card>
              ))}
            </div>
        </div>

        <Dialog open={editOpen} onOpenChange={setEditOpen}>
          <DialogContent>
            <DialogHeader>
              <DialogTitle>Edit App</DialogTitle>
              <DialogDescription>
                Update the application details.
              </DialogDescription>
            </DialogHeader>
            <div className="grid gap-4 py-4">
              <div className="grid gap-2">
                <Label htmlFor="edit-name">Name</Label>
                <Input
                  id="edit-name"
                  value={formData.name}
                  onChange={(e) => setFormData({ ...formData, name: e.target.value })}
                  placeholder="App name"
                />
              </div>
              <div className="grid gap-2">
                <Label htmlFor="edit-url">URL</Label>
                <Input
                  id="edit-url"
                  value={formData.url}
                  onChange={(e) => setFormData({ ...formData, url: e.target.value })}
                  placeholder="https://example.com"
                />
              </div>
              <div className="grid gap-2">
                <Label htmlFor="edit-icon">Icon (optional)</Label>
                <input
                  id="edit-icon"
                  type="file"
                  accept="image/*"
                  onChange={async (e) => {
                    const file = e.target.files?.[0];
                    if (file) {
                      const path = await invoke<string>("get_file_path", { file: file.name });
                      setFormData({ ...formData, icon: path });
                    }
                  }}
                  className="hidden"
                />
                <label htmlFor="edit-icon" className="cursor-pointer">
                  {formData.icon ? (
                    <img 
                      src={imageSrcs[formData.name] || convertFileSrc(formData.icon)} 
                      alt="App icon" 
                      className="w-24 h-24 rounded-lg object-cover hover:opacity-80 transition-opacity"
                    />
                  ) : (
                    <div className="w-24 h-24 rounded-lg border-2 border-dashed border-gray-300 flex items-center justify-center hover:border-gray-400 transition-colors">
                      <Plus className="h-8 w-8 text-gray-400" />
                    </div>
                  )}
                </label>
              </div>
            </div>
            <DialogFooter>
              <Button variant="outline" onClick={() => setEditOpen(false)}>
                Cancel
              </Button>
              <Button onClick={handleEdit}>Save Changes</Button>
            </DialogFooter>
          </DialogContent>
        </Dialog>

        <Dialog open={deleteOpen} onOpenChange={setDeleteOpen}>
          <DialogContent>
            <DialogHeader>
              <DialogTitle>Delete App</DialogTitle>
              <DialogDescription>
                Are you sure you want to delete "{selectedApp?.name}"? This action cannot be undone.
              </DialogDescription>
            </DialogHeader>
            <DialogFooter>
              <Button variant="outline" onClick={() => setDeleteOpen(false)}>
                Cancel
              </Button>
              <Button variant="destructive" onClick={handleDelete}>
                Delete
              </Button>
            </DialogFooter>
          </DialogContent>
        </Dialog>
      </div>
  );
}

export default App;
