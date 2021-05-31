import { ComponentFixture, TestBed } from '@angular/core/testing';

import { BrowseDirectoryComponent } from './browse-directory.component';

describe('BrowseDirectoryComponent', () => {
  let component: BrowseDirectoryComponent;
  let fixture: ComponentFixture<BrowseDirectoryComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      declarations: [ BrowseDirectoryComponent ]
    })
    .compileComponents();
  });

  beforeEach(() => {
    fixture = TestBed.createComponent(BrowseDirectoryComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
