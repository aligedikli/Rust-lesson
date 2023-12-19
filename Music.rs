struct Music {
    name: String,
    artist: String,
    liked: bool,
}

impl Music {
    fn new(name: &str, artist: &str) -> Self {
        Music {
            name: name.to_string(),
            artist: artist.to_string(),
            liked: false,
        }
    }

    fn like(&mut self) {
        self.liked = true;
    }

    fn unlike(&mut self) {
        self.liked = false;
    }

    fn is_liked(&self) -> bool {
        self.liked
    }
}

struct Playlist {
    name: String,
    musics: Vec<Music>,
}

impl Playlist {
    fn new(name: &str) -> Self {
        Playlist {
            name: name.to_string(),
            musics: Vec::new(),
        }
    }

    fn add_music(&mut self, music: Music) -> Result<(), &str> {
        if self.musics.iter().any(|m| m.name == music.name) {
            return Err("A song with the same name already exists in the playlist.");
        }
        self.musics.push(music);
        Ok(())
    }

    fn remove_music(&mut self, music_name: &str) -> Result<(), &str> {
        if let Some(index) = self.musics.iter().position(|m| m.name == music_name) {
            self.musics.remove(index);
            Ok(())
        } else {
            Err("Song not found in the playlist.")
        }
    }

    fn set_now_playing(&self, music_name: &str) -> Result<(), &str> {
        if let Some(index) = self.musics.iter().position(|m| m.name == music_name) {
            println!("Now playing: {} - {}", self.musics[index].name, self.musics[index].artist);
            Ok(())
        } else {
            Err("Song not found in the playlist.")
        }
    }

    fn like_song_in_playlist(&mut self, music_name: &str) -> Result<(), &str> {
        if let Some(music) = self.musics.iter_mut().find(|m| m.name == music_name) {
            music.like();
            Ok(())
        } else {
            Err("Song not found in the playlist.")
        }
    }

    fn unlike_song_in_playlist(&mut self, music_name: &str) -> Result<(), &str> {
        if let Some(music) = self.musics.iter_mut().find(|m| m.name == music_name) {
            music.unlike();
            Ok(())
        } else {
            Err("Song not found in the playlist.")
        }
    }

    fn show_playlist(&self) {
        println!("Playlist: {}", self.name);
        for music in &self.musics {
            println!("{} - {} Liked: {}", music.name, music.artist, music.is_liked());
        }
    }
}

fn main() {
    let mut playlist = Playlist::new("My Playlist");

    let song1 = Music::new("Song 1", "Artist 1");
    let song2 = Music::new("Song 2", "Artist 2");
    let song3 = Music::new("Song 3", "Artist 3");

    playlist.add_music(song1).unwrap();
    playlist.add_music(song2).unwrap();
    playlist.add_music(song3).unwrap();

    playlist.set_now_playing("Song 1").unwrap();

    // Beğenilen şarkıları işaretleyelim
    playlist.like_song_in_playlist("Song 1").unwrap();
    playlist.like_song_in_playlist("Song 3").unwrap();

    // Beğenilmeyen şarkıları işaretleyelim
    playlist.unlike_song_in_playlist("Song 2").unwrap();

    // Çalma listesini gösterelim
    playlist.show_playlist();
}
